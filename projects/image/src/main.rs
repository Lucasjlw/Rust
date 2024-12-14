use flate2::write::ZlibEncoder;
use flate2::Compression;
use rand::RngCore;
use std::io::Write;

fn main() {
    let mut image = PNG::new(250, 250);
    image.scramble_data();
    image.write_to_file("generated/generated2.png");
}

struct PNG {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

enum ChunkType {
    IHDR,
    IDAT,
    IEND,
}

impl PNG {
    fn from(data: Vec<u8>) -> Self {
        PNG {
            data,
            width: 0,
            height: 0,
        }
    }

    fn new(width: u32, height: u32) -> Self {
        PNG {
            data: Vec::from(Self::generate_IHDR_chunk(width, height, 0)),
            width,
            height,
        }
    }

    fn load_from_file(path: &str) -> Self {
        let buffer = std::fs::read(path).expect("Error reading file");

        Self::from(buffer)
    }

    fn write_to_file(&self, path: &str) {
        std::fs::write(path, self.data.clone());
    }

    fn scramble_data(&mut self) {
        self.data = vec![137, 80, 78, 71, 13, 10, 26, 10];
        self.data
            .extend(Self::generate_IHDR_chunk(self.width, self.height, 0));

        self.data
            .extend(Self::generate_IDAT_chunk(self.width, self.height));

        self.data.extend(Self::generate_IEND_chunk());
    }

    fn generate_IHDR_chunk(width: u32, height: u32, color_type: usize) -> [u8; 25] {
        let mut chunk = [0u8; 25];

        // Length is 13
        chunk[0..4].copy_from_slice(&[0x00, 0x00, 0x00, 0x0D]);

        // Type is IHDR
        chunk[4..8].copy_from_slice(&[0x49, 0x48, 0x44, 0x52]);

        // Width in 4 bytes
        chunk[8..12].copy_from_slice(&width.to_be_bytes());

        // Height in 4 bytes
        chunk[12..16].copy_from_slice(&height.to_be_bytes());

        // Bit depth, color type, compression, filter, and interlace
        chunk[16..21].copy_from_slice(&[0x04, 0x00, 0x00, 0x00, 0x00]);

        let crc = {
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(&chunk[4..21]);
            hasher.finalize()
        };

        // CRC - TODO; random for now
        chunk[21..25].copy_from_slice(&crc.to_be_bytes());

        chunk
    }

    fn generate_IDAT_chunk(width: u32, height: u32) -> Vec<u8> {
        let mut raw_data: Vec<u8> = Vec::new();

        // Calculate padded width in bits (must be multiple of 8)
        let line_width_bits = width * 4;
        let line_width_bytes = (line_width_bits + 7) / 8; // Round up to nearest byte

        // For each scanline
        for _ in 0..height {
            // Add filter byte at start of scanline
            raw_data.push(0);

            // Add pixel data for this scanline
            for byte_index in 0..line_width_bytes {
                let mut byte = 0u8;

                // Pack 8 pixels into one byte
                for pixel in 0..2 {
                    if byte_index * 2 + pixel < width {
                        // Get value between 0 and 15
                        let pixel_value = (rand::thread_rng().next_u32() % 16) as u8;
                        byte |= pixel_value << (4 - (pixel * 4)); // Shift into position
                    }
                }
                raw_data.push(byte);
            }
        }

        // Compress the data using zlib
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&raw_data).unwrap();
        let compressed_data = encoder.finish().unwrap();

        let mut chunk: Vec<u8> = Vec::new();

        // Length of compressed data
        chunk.extend_from_slice(&(compressed_data.len() as u32).to_be_bytes());

        // Chunk type
        chunk.extend_from_slice(b"IDAT");

        // Compressed data
        chunk.extend_from_slice(&compressed_data);

        // Calculate CRC
        let crc = {
            let mut hasher = crc32fast::Hasher::new();
            hasher.update(&chunk[4..]);
            hasher.finalize()
        };

        chunk.extend_from_slice(&crc.to_be_bytes());

        chunk
    }

    fn generate_IEND_chunk() -> [u8; 12] {
        [
            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ]
    }
}
