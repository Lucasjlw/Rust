use std::collections::VecDeque;

pub struct LZ77 {}

impl LZ77 {
    pub fn compress(data: &Vec<u8>) -> Vec<u8> {
        let mut window: VecDeque<u8> = VecDeque::new();
        let mut result: Vec<u8> = vec![];
        let window_size = 65535;
        let mut pos = 0;

        while pos < data.len() {
            let mut best_length = 0;
            let mut best_distance = 0;
            
            // Look for matches in the sliding window
            let search_start = if pos >= window_size { pos - window_size } else { 0 };
            for start in search_start..pos {
                let mut length = 0;
                // Check how many bytes match starting at this position
                while pos + length < data.len() && 
                      length < 255 && 
                      data[start + length] == data[pos + length] {
                    length += 1;
                }
                
                if length > best_length {
                    best_length = length;
                    best_distance = pos - start;
                }
            }

            if best_length > 3 {
                // Output match
                result.push(1);
                // Store distance as two bytes (little-endian)
                result.push((best_distance & 0xFF) as u8);
                result.push(((best_distance >> 8) & 0xFF) as u8);
                result.push(best_length as u8);
                pos += best_length;
            } else {
                // Output literal
                result.push(0);
                result.push(data[pos]);
                pos += 1;
            }

            // Update window
            while window.len() >= window_size {
                window.pop_front();
            }
            window.extend(&data[if pos >= window_size { pos - window_size } else { 0 }..pos]);
        }

        result
    }
}
