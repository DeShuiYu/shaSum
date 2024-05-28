use std::env::args;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::exit;
use sha2::{Digest, Sha512};
use sha2::digest::Update;

fn main() {
    let args:Vec<String> = args().collect();
    if args.len() < 2{
        eprintln!("need input filename or filepath");
        exit(1);
    }
    let file = File::open(&args[1]).unwrap();
    let mut reader =BufReader::new(file);
    let mut hasher = Sha512::new();
    let mut buffer = [0;1024];
    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0{
            break;
        }
        Update::update(&mut hasher,&buffer[..bytes_read]);
    }
    println!("{:x}\t{:?}",hasher.finalize(),args[1]);
}
