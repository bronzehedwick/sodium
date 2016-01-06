use editor::Editor;
use redraw::RedrawTask;
use buffer::{Buffer, Line};

impl<'a, B: Buffer<'a>> Editor<B> {
    /// Delete a character
    #[inline]
    pub fn delete(&mut self) {
        let (x, y) = self.pos();
        if x == 0 {
            if y != 0 {
                let s = self.buffer.remove_line(y);
                self.buffer.get_line_mut(y - 1).push_slice(s.as_slice());
                let len = self.buffer.get_line(y - 1).len();
                self.goto((len, y - 1));
                self.redraw_task = RedrawTask::Lines(y..y + 1);
            }
        } else if x < self.buffer.get_line(y).len() {
            self.buffer.get_line_mut(y).remove(x);
            self.redraw_task = RedrawTask::LinesAfter(y);
        }

        self.hint();
    }
}
