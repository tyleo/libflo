use std::io::Cursor;
use std::io::Read;
use std::io::Result;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct ReadingEnd {
    locking_stream: Arc<Mutex<Cursor<Vec<u8>>>>,
    position: u64
}

impl ReadingEnd {
    pub fn new(locking_stream: Arc<Mutex<Cursor<Vec<u8>>>>) -> ReadingEnd {
        ReadingEnd{ locking_stream: locking_stream, position: 0 }
    }

    fn get_cursor(&self) -> MutexGuard<Cursor<Vec<u8>>> {
        self.locking_stream.lock().unwrap()
    }

    fn exec<TFn, TOut>(&mut self, exec_fn: TFn) -> Result<TOut>
        where TFn: FnOnce(&mut Read) -> Result<TOut> {
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

impl Read for ReadingEnd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.exec(|read| read.read(buf))
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.exec(|read| read.read_to_end(buf))
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        self.exec(|read| read.read_to_string(buf))
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.exec(|read| read.read_exact(buf))
    }
}
