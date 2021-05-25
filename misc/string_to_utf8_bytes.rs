// Quick and dirty commandline to tool print the UTF-8 bytes in hex for a arg
use std::env;
use std::process;

fn main() {
    fn print_usage() {
        println!("string_to_utf8_bytes <STRING>");
    };

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    let s = &args[1].to_string();

    for b in s.as_bytes() {
        print!("0X{:02X},", b);
    }
    println!();
}
