use std::process::exit;

const DEFAULT_BITS: usize = 16;

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
    println!("{}", geohash_rs::encode(lat, lng, bits));
}
