use std::fmt::Arguments;
use std::io::Result;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct CloneWriter<T> {
    writer: Arc<Mutex<T>>,
}

impl <T> CloneWriter<T> {
    pub fn new(writer: T) -> Self {
        let writer = Arc::new(Mutex::new(writer));
        CloneWriter{ writer: writer }
    }

    fn get_writer(&self) -> MutexGuard<T> {
        self.writer.lock().unwrap()
    }
}

impl <T> Clone for CloneWriter<T> {
    fn clone(&self) -> Self {
        CloneWriter {
            writer: self.writer.clone()
        }
    }
}

impl <T> Write for CloneWriter<T>
    where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.get_writer().write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.get_writer().flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.get_writer().write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments) -> Result<()> {
        self.get_writer().write_fmt(fmt)
    }
}
