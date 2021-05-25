// commandline tool for megadrive-cart-info

use megadrive_cart_info::megadrive::MegaDriveROMHeader;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    fn print_usage() {
        println!("megadrive_cart_info <ARGS>");
    };

    let args: Vec<String> = env::args().collect();

    //println!("Len = {}", args.len());

    if args.len() != 2 {
        print_usage();
        process::exit(1);
    }

    let filename = &args[1];
    //println!("Got {}", filename);

    let a = MegaDriveROMHeader::new_from_file(Path::new(filename)).unwrap();

    // Default print output - trim whitespace, use hex
    println!("filename: {}", filename);
    println!("system name: {}", a.system_name.trim());
    println!("copyright notice: {}", a.copyright_notice.trim());

    println!("game name domestic: {}", a.game_name_domestic.trim());
    println!("game name overseas: {}", a.game_name_overseas.trim());
    println!("product identifier: {}", a.product_identifier.trim());
    println!("checksum: {:#06X}", a.checksum);
    println!("device_support: {}", a.device_support.trim());
    println!("rom_start: {:#010X}", a.rom_start);
    println!("rom_end: {:#010X}", a.rom_end);
    println!("ram_start: {:#010X}", a.ram_start);
    println!("ram_end: {:#010X}", a.ram_end);
    println!("extra_memory: {}", a.extra_memory);
    println!("extra_memory_type: {:#04X}", a.extra_memory_type);
    println!("extra_memory_start: {:#010X}", a.extra_memory_start);
    println!("extra_memory_end: {:#010X}", a.extra_memory_end);
    println!("modem support: {}", a.modem_support);
    println!("memo: {}", a.memo.trim());
    println!("region: {}", a.region.trim());
}
