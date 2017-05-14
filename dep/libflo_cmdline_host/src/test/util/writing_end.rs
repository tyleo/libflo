use std::fmt::Arguments;
use std::io::Cursor;
use std::io::Result;
use std::io::Write;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct WritingEnd {
    locking_stream: Arc<Mutex<Cursor<Vec<u8>>>>,
    position: u64
}

impl WritingEnd {
    pub fn new(locking_stream: Arc<Mutex<Cursor<Vec<u8>>>>) -> WritingEnd {
        WritingEnd{ locking_stream: locking_stream, position: 0 }
    }

    fn get_cursor(&self) -> MutexGuard<Cursor<Vec<u8>>> {
        self.locking_stream.lock().unwrap()
    }

    fn exec<TFn, TOut>(&mut self, exec_fn: TFn) -> Result<TOut>
        where TFn: FnOnce(&mut Write) -> Result<TOut> {
        let (result, new_position) = {
            let mut cursor = self.get_cursor();
            cursor.set_position(self.position);
            let result = exec_fn(cursor.deref_mut());
            let position = cursor.position();
            (result, position)
        };
        self.position = new_position;
        result
    }
}

impl Write for WritingEnd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.exec(|write| write.write(buf))
    }

    fn flush(&mut self) -> Result<()> {
        self.exec(|write| write.flush())
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.exec(|write| write.write_all(buf))
    }

    fn write_fmt(&mut self, fmt: Arguments) -> Result<()> {
        self.exec(|write| write.write_fmt(fmt))
    }
}
