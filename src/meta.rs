
pub type ConnectionID = usize;
pub type WindowID = usize;

// TODO: Consider how specification of the arguments for commands ought to work, or if it ought
// to be a thing in the first place.
pub type Command = String;

// This is the Event type, e.g., fragments of data about something that's happened--user input,
// text sent by a remote server, etcetera. They get generated by code running in threads, which are
// managed by some object implementing the EventManager trait below.
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
    fn add(&mut self, src: EventSource);
    fn next_event(&mut self) -> Event;
}

// This object knows about the logistical details of handling UI, like drawing to the screens.
// It should always impl EventSource.  It needs to generate Events, such as when text input is
// sent from the input pane.  Technically, it can react to some inputs on its own and only
// generate Events for what the rest of the system needs to know about.
pub trait UserInterface {
    // The way windows work is that any unique named window you try to send text to should be
    // created by the UI code. Which windows are visible at any given time, and how that activity
    // is surface to the user, is the UI code's business.
    fn push_to_window(&mut self, window: String, line: String) -> Result<(), ()>;
    fn register_command(&mut self, c: Command);
}

// This type of object knows about servers and contains the low-level logic for connecting and
// listening to a particular sort of MUD server.  These objects should always impl EventSource.
//
// The `address' is provided in a single String with an implementation-defined format to
// accomodate those types of server that may not be able to be satisfied with a traditional
// host/port pair.
pub trait ConnectionInterface {
    fn start_connection(&mut self, address: String) -> ConnectionID;
    fn stop_connection(&mut self, which: ConnectionID) -> Result<(), ()>;
}

