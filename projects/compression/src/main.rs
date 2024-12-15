mod lz77;

use lz77::LZ77;

fn main() {
    let data: Vec<u8> = Vec::from("12312333223322123".as_bytes());
    

    let compressed = LZ77::compress(&data.to_vec());

    println!("original length: {}, compressed length: {}", data.len(), compressed.len());

    println!("{:?}", std::str::from_utf8(&compressed).unwrap());
}