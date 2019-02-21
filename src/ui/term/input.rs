
// TODO: Shouldn't there be a prettier way to import this?
use crate::ui::term::Window;
use crate::utils::force_width;

/// UI for input/editing of a single line of text on the terminal.
pub struct InputLine {
    // We could have used a more clever data structure, but as best I could tell from a cursory
    // attempt to look through tinyfugue's source they're not doing anything more clever than
    // shuffling memory around, either.  Maybe we'll need/want to upgrade, but we can start simple
    // and see if it performs unacceptably for the kind of editing we need to do.
    //
    // TODO: Should this be Vec<String> later because of unicode?  char is probably faster for now
    // though.
    buffer: Vec<char>,
    // The cursor is 0-indexed... but keep in mind that we usually think of a cursor as BETWEEN two
    // characters.
    cursor: usize,
    target_width: usize,
}

impl Window for InputLine {
    fn render(&self) -> Vec<String> {
        // Split the buffer up into chunks of size `target_width`, turn them into strings and
        // force_width() them.
        self.buffer.chunks(self.target_width).map(|chunk| {
            let mut chunk: String = chunk.iter().collect();
            force_width(chunk, self.target_width)
        }).collect()
    }

    fn get_size(&self) -> (usize, usize) {
        // This is probably stupid, but casting to a float and using ceil seemed even more stupid.
        let mut lines: usize = self.buffer.len() / self.target_width;
        let remainder: usize = self.buffer.len() % self.target_width;
        if remainder > 0 || lines == 0 {
            lines += 1;
        }
        (self.target_width, lines)
    }

    fn get_cursor_pos(&self) -> (usize, usize) {
        let x: usize = self.cursor % self.buffer.len();
        let y: usize = self.cursor / self.buffer.len();

        (x, y)
    }

    fn set_width(&mut self, new_w: usize) {
        self.target_width = new_w;
    }

    fn set_height(&mut self, _new_h: usize) {
        panic!("Can't set the height of an InputLine: It's derived dynamically.");
    }
}

impl InputLine {
    pub fn new(width: usize, _height: usize) -> InputLine {
        InputLine {
            buffer: vec![],
            cursor: 0,
            target_width: width,
        }
    }

    /// Insert a single character at the current cursor position.
    pub fn insert_char(&mut self, what: char) {
        // The cursor is considered to be between two characters.  So, taken as an array index, it
        // will point to the character directly after itself, unless it's at the end, in which case
        // using it like an index will probably cause a panic.
        if self.cursor >= self.buffer.len() {
            self.buffer.push(what);
            self.cursor = self.buffer.len();
        } else {
            self.buffer.insert(self.cursor, what);
            self.cursor += 1;
        }
    }

    /// Set the contents of the input to some String.
    pub fn set_string(&mut self, what: String) {
        self.buffer = what.chars().collect();
    }

    /// Move the cursor `offset` chars to the left or right in the buffer, not allowing it to go
    /// out-of-bounds.
    pub fn move_cursor(&mut self, offset: isize) {
        if offset.is_negative() {
            let backwards = offset.abs() as usize;
            self.cursor = if backwards > self.cursor {
                0
            } else {
                self.cursor - backwards
            };
        } else {
            self.cursor += offset as usize;
            if self.cursor > self.buffer.len() {
                self.cursor = self.buffer.len();
            }
        }
    }

    fn as_text(&self) -> String {
        let result: String = self.buffer.iter().collect();
        result
    }
}
