use std::process::exit;

const DEFAULT_BITS: usize = 16;
const BITS_PER_BYTE: usize = 8;

fn main() {
    if std::env::args().len() != 3 && std::env::args().len() != 4 {
        println!(
            "usage: geohash latitude longitude [bits (default = {})]",
            DEFAULT_BITS
        );
        exit(1);
    }
    let (lat, lng, bits) = {
        let mut args = std::env::args();
        args.next(); // drop the first argument (command name)
        let lat = args.next().unwrap().parse::<f64>();
        let lng = args.next().unwrap().parse::<f64>();
        let bits = match args.next() {
            Some(arg) => arg.parse::<usize>(),
            None => Ok(DEFAULT_BITS),
        };
        if let Err(e) = lat {
            println!("failed to parse latitude. {}", e);
            exit(1);
        }
        if let Err(e) = lng {
            println!("failed to parse longitude. {}", e);
            exit(1);
        }
        if let Err(e) = bits {
            println!("failed to parse bits. {}", e);
            exit(1);
        }
        (lat.unwrap(), lng.unwrap(), bits.unwrap())
    };
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
        println!(
            "[i={:04}, j={:04}] ptr={:08b}, byte={:08b}, lat_ptr={:08b}, lng_ptr={:08b}",
            i, j, ptr, byte, lat_ptr, lng_ptr
        );
        *ptr = byte;
    }
    for b in &bytes {
        print!("{:08b} ", b);
    }
    println!();
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
        println!(
            "[iter={:04}] ptr={:08b}, byte={:08b}, lower={:08}, higher={:08}, piv={:08}",
            i, ptr, byte, lower, higher, piv
        );
        *ptr = byte;
        piv = (lower + higher) / 2.0;
    }
    buf
}
