use std::{env, fs::File, io::Read};

fn main() {
    // Determine which file to execute
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: bf <file.bf>");
        std::process::exit(1);
    }

    let filename = &args[1];

    // Read file
    let mut file = File::open(filename).expect("program file not found");
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read program file");

    bf_rust::fuck(source);
}
