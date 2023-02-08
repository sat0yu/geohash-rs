use std::{collections::HashMap, process::exit};

const BITS_PER_BYTE: usize = 8;

pub fn encode(lat: f64, lng: f64, bits: usize) -> String {
    let lat_bytes = trace_binary_search(lat, (-90.0, 90.0), bits);
    let lng_bytes = trace_binary_search(lng, (-180.0, 180.0), bits);
    let mut bytes = vec![0u8; lat_bytes.len() * 2];
    for n in 0..bits {
        let i = n / BITS_PER_BYTE;
        let p = n % BITS_PER_BYTE;
        let lat_ptr = lat_bytes.get(i).unwrap();
        let lng_ptr = lng_bytes.get(i).unwrap();
        let lat_b = (*lat_ptr >> (BITS_PER_BYTE - p - 1)) & 1u8;
        let lng_b = (*lng_ptr >> (BITS_PER_BYTE - p - 1)) & 1u8;
        let j = (n * 2) / BITS_PER_BYTE;
        let ptr = bytes.get_mut(j).unwrap();
        let mut byte = (*ptr) << 2;
        byte |= lng_b << 1 | lat_b;
        // println!(
        //     "[i={:04}, j={:04}] ptr={:08b}, byte={:08b}, lat_ptr={:08b}, lng_ptr={:08b}",
        //     i, j, ptr, byte, lat_ptr, lng_ptr
        // );
        *ptr = byte;
    }
    // for b in &bytes {
    //     print!("{:08b} ", b);
    // }
    // println!();
    base32encode(&bytes)
}

fn base32encode(bytes: &[u8]) -> String {
    let map: HashMap<u8, char> = HashMap::from([
        (0b00000, '0'),
        (0b00001, '1'),
        (0b00010, '2'),
        (0b00011, '3'),
        (0b00100, '4'),
        (0b00101, '5'),
        (0b00110, '6'),
        (0b00111, '7'),
        (0b01000, '8'),
        (0b01001, '9'),
        (0b01010, 'b'),
        (0b01011, 'c'),
        (0b01100, 'd'),
        (0b01101, 'e'),
        (0b01110, 'f'),
        (0b01111, 'g'),
        (0b10000, 'h'),
        (0b10001, 'j'),
        (0b10010, 'k'),
        (0b10011, 'm'),
        (0b10100, 'n'),
        (0b10101, 'p'),
        (0b10110, 'q'),
        (0b10111, 'r'),
        (0b11000, 's'),
        (0b11001, 't'),
        (0b11010, 'u'),
        (0b11011, 'v'),
        (0b11100, 'w'),
        (0b11101, 'x'),
        (0b11110, 'y'),
        (0b11111, 'z'),
    ]);
    let mut result = String::new();
    let mut t = 0u8;
    for n in 0..(bytes.len() * BITS_PER_BYTE) {
        let i = n / BITS_PER_BYTE;
        let p = n % BITS_PER_BYTE;
        let ptr = bytes.get(i).unwrap();
        t = (t << 1) | ((*ptr) >> (BITS_PER_BYTE - p - 1)) & 1u8;
        if (n + 1) % 5 == 0 {
            result.push(*map.get(&t).unwrap());
            t = 0;
        }
    }
    result
}

fn trace_binary_search(value: f64, range: (f64, f64), bits: usize) -> Vec<u8> {
    if range.0 >= range.1 {
        println!("invalid range. {:?}", range);
        exit(1);
    }
    if value < range.0 || range.1 < value {
        println!("insufficient range. {:?}", range);
        exit(1);
    }
    let len = (bits as f64 / BITS_PER_BYTE as f64).ceil() as usize;
    let mut buf = vec![0u8; len];
    let mut lower = range.0;
    let mut higher = range.1;
    let mut piv = (range.0 + range.1) / 2.0;
    for i in 0..bits {
        let ptr = buf.get_mut(i / BITS_PER_BYTE).unwrap();
        let mut byte = (*ptr) << 1;
        if value - piv < 0.0 {
            higher = piv;
        } else {
            byte |= 1u8;
            lower = piv;
        }
        // println!(
        //     "[iter={:04}] ptr={:08b}, byte={:08b}, lower={:08}, higher={:08}, piv={:08}",
        //     i, ptr, byte, lower, higher, piv
        // );
        *ptr = byte;
        piv = (lower + higher) / 2.0;
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_translates_latitude_and_longitude_to_geohash() {
        let cases = vec![
            ((57.64911, 10.40744, 8), "u4p".to_string()),
            ((57.64911, 10.40744, 12), "u4pru0".to_string()),
            ((57.64911, 10.40744, 16), "u4pruy".to_string()),
            ((57.64911, 10.40744, 32), "u4pruydqqvj8".to_string()),
        ];
        for ((lat, lng, bits), expected) in cases {
            assert_eq!(encode(lat, lng, bits), expected);
        }
    }

    #[test]
    fn base32encode_converts_bytes_to_32ghs_string() {
        let cases = vec![
            (vec![0b01101111, 0b11110000, 0b01000001], "ezs4".to_string()),
            (
                vec![0b01101111, 0b11110000, 0b01000001, 0b00000000],
                "ezs420".to_string(),
            ),
            (
                vec![0b01101111, 0b11110000, 0b01000001, 0b01000100],
                "ezs42j".to_string(),
            ),
            (
                // the trailing last n-bits (n < 5) are ignored
                vec![0b01101111, 0b11110000, 0b01000001, 0b01000111],
                "ezs42j".to_string(),
            ),
            (
                vec![0b01101111, 0b11110000, 0b01000001, 0b01000111, 0b00011111],
                "ezs42jsz".to_string(),
            ),
        ];
        for (bytes, expected) in cases {
            assert_eq!(base32encode(&bytes), expected);
        }
    }

    #[test]
    fn race_binary_search_returns_trace_of_binary_search() {
        let cases = vec![
            ((42.6, (-90.0, 90.0), 8), vec![0b10111100]),
            ((-5.6, (-180.0, 180.0), 8), vec![0b01111100]),
            ((42.6, (-90.0, 90.0), 12), vec![0b10111100, 0b10010000]),
            ((-5.6, (-180.0, 180.0), 12), vec![0b01111100, 0b00000000]),
            ((42.6, (-90.0, 90.0), 16), vec![0b10111100, 0b10010110]),
            ((-5.6, (-180.0, 180.0), 16), vec![0b01111100, 0b00000100]),
        ];
        for ((value, range, bits), expected) in cases {
            assert_eq!(trace_binary_search(value, range, bits), expected);
        }
    }
}
