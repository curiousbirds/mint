
pub type ConnectionID = usize;
pub type WindowID = usize;

// TODO: Consider how specification of the arguments for commands ought to work, or if it ought
// to be a thing in the first place.
pub type Command = String;

// This is the Event type, e.g., fragments of data about something that's happened--user input,
// text sent by a remote server, etcetera. They get generated by code running in threads, which are
// managed by some object implementing the EventManager trait below.
#[derive(Debug)]
pub enum Event {
    UserCommand { cmd: Command },
    // We will want to be able to discriminate which _window_ in the UI a line of text came from,
    // not which connection it should go to.  That is, the UI doesn't know anything about the
    // mapping of windows to connections.)
    UserInput { line: String, which: WindowID },

    ServerText { line: String, which: ConnectionID },
    ConnectionStart { which: ConnectionID },
    ConnectionEnd { which: ConnectionID, reason: String },
}

// Objects that generate Events impl this.
pub trait EventSource {
    fn run(&mut self, channel: std::sync::mpsc::Sender<Event>);
}

// This object, as noted above, deals with managing sources of Events.
pub trait EventManager {
    fn start_source(&mut self, src: Box<EventSource + Send>);
    fn next_event(&mut self) -> Result<Event, String>;
}

// This object knows about the logistical details of handling UI, like drawing to the screens.
//
// It needs to generate Events, such as when text input is sent from the input pane.  Technically,
// it can react to some inputs on its own and only generate Events for what the rest of the system
// needs to know about.  The problem with impl'ing EventSource on this object is that EventSources
// are consumed and moved into another thread by the EventManager, and this object also needs to be
// sent data from the main thread; to fix this, we generate a subsidiary EventSource object
// instead.
pub trait UserInterface {
    // The way windows work is that any unique named window you try to send text to should be
    // created by the UI code. Which windows are visible at any given time, and how that activity
    // is surface to the user, is the UI code's business.
    fn push_to_window(&mut self, window: String, line: String) -> Result<(), ()>;
    fn register_command(&mut self, c: Command);

    fn listener(&mut self) -> Box<EventSource + Send>;
}

// This type of object knows about servers and contains the low-level logic for connecting and
// listening to a particular sort of MUD server.  It returns a secondary object instead of directly
// impl'ing EventSource for the reasons listed above.
//
// The `address' is provided in a single String with an implementation-defined format to
// accomodate those types of server that may not be able to be satisfied with a traditional
// host/port pair.
pub trait ConnectionInterface {
    fn start_connection(&mut self, address: String) -> Result<ConnectionID, String>;
    fn stop_connection(&mut self, which: ConnectionID) -> Result<(), ()>;
    fn write_to_connection(&mut self, which: ConnectionID, what: String) -> Result<(), ()>;

    fn listener(&mut self) -> Box<EventSource + Send>;
}

