use std::fs::File;
use std::io::prelude::*;
use std::env;
use fled::parser::parse_flac_stream;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No filename provided.");
        return;
    }
    let filename = args[1].clone();
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let flac_stream = parse_flac_stream::<nom::error::Error<(&[u8], usize)>>((contents.as_slice(), 0));
    match flac_stream {
        Ok(((_, o), s)) => {
            println!("FLAC Stream: {:?}", s);
            println!("Current bit offset: {}", o);
        },
        Err(e) => {
            println!("Parse failed: {:?}", e);
        }
    }
}
