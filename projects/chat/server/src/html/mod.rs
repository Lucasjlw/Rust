use std::error::Error;
use std::io::Read;
use std::fs::read;

pub struct HtmlReader {
    data: Vec<u8>,
}

impl HtmlReader {
    pub fn read_file(path: &str, cursor: usize) -> Result<Vec<u8>, impl Error> {
        let result = read(path);

        return result;
    }
}
