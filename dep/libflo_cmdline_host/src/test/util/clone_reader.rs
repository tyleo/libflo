use std::io::Read;
use std::io::Result;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct CloneReader<T> {
    reader: Arc<Mutex<T>>,
}

impl <T> CloneReader<T> {
    pub fn new(reader: T) -> Self {
        let reader = Arc::new(Mutex::new(reader));
        CloneReader{ reader: reader }
    }

    fn get_reader(&self) -> MutexGuard<T> {
        self.reader.lock().unwrap()
    }
}

impl <T> Clone for CloneReader<T> {
    fn clone(&self) -> Self {
        CloneReader {
            reader: self.reader.clone()
        }
    }
}

impl <T> Read for CloneReader<T>
    where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.get_reader().read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.get_reader().read_to_end(buf)
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        self.get_reader().read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.get_reader().read_exact(buf)
    }
}
