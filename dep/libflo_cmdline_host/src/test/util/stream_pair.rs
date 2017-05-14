use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;
use test::util::ReadingEnd;
use test::util::WritingEnd;

pub struct StreamPair {
    locking_stream: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl StreamPair {
    pub fn new() -> StreamPair {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let mutex = Mutex::new(cursor);
        let arc = Arc::new(mutex);
        StreamPair{ locking_stream: arc }
    }

    pub fn to_pair(self) -> (ReadingEnd, WritingEnd) {
        let locking_stream = self.locking_stream;
        (
            ReadingEnd::new(locking_stream.clone()),
            WritingEnd::new(locking_stream.clone())
        )
    }
}
