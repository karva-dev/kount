use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn count_lines(path: &Path) -> io::Result<u64> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(64 * 1024, file);
    let mut count = 0u64;

    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break;
        }
        count += bytecount::count(buf, b'\n') as u64;
        let len = buf.len();
        reader.consume(len);
    }

    Ok(count)
}
