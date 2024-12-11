use std::error::Error;
use std::io::Read;

pub struct HtmlReader {
    data: Vec<u8>,
}

impl HtmlReader {
    fn read_file(path: &str, cursor: usize) -> Result<Vec<u8>, impl Error> {}
}
