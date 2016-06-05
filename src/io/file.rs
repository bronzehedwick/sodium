use edit::buffer::{TextBuffer, SplitBuffer};
use state::editor::{Buffer, Editor};
use std::fs::File;
use std::io::{Read, Write};

/// The status of a file IO operation.
pub enum FileStatus {
    /// Oll fino.
    Ok,
    /// File not found.
    NotFound,
    /// Other error.
    Other,
}

impl Editor {
    /// Open a file.
    pub fn open(&mut self, path: &str) -> FileStatus {
        if let Some(mut file) = File::open(path).ok() {
            let mut con = String::new();
            let _ = file.read_to_string(&mut con);

            let mut new_buffer: Buffer = SplitBuffer::from_str(&con).into();
            new_buffer.title = Some(path.into());

            let new_buffer_index = self.buffers.new_buffer(new_buffer);
            self.buffers.switch_to(new_buffer_index);
            self.hint();
            FileStatus::Ok
        } else {
            FileStatus::NotFound
        }
    }

    /// Write the file.
    pub fn write(&mut self, path: &str) -> FileStatus {
        self.buffers.current_buffer_info_mut().title = Some(path.into());
        if let Some(mut file) = File::create(path).ok() {
            if file.write(self.buffers.current_buffer().to_string().as_bytes()).is_ok() {
                FileStatus::Ok
            } else {
                FileStatus::Other
            }
        } else {
            FileStatus::NotFound
        }
    }
}