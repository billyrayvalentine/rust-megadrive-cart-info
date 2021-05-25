/// A struct to represent a Sega MegaDrive ROM header
/// Example to construct TODO
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

use crate::shift_jis;

const ROM_START: i16 = 0x100;
const ROM_HEADER_SIZE: i16 = 0x100;

#[derive(Debug, Eq, PartialEq)]
pub struct MegaDriveROMHeader {
    // https://plutiedev.com/rom-header
    // These should probably be Options - values with spaces can be Option::None
    pub system_name: String,
    pub copyright_notice: String,
    pub game_name_domestic: String,
    pub game_name_overseas: String,
    pub product_identifier: String,
    pub checksum: u16,
    pub device_support: String,
    pub rom_start: u32,
    pub rom_end: u32,
    pub ram_start: u32,
    pub ram_end: u32,
    pub extra_memory: bool,
    pub extra_memory_type: u8,
    pub extra_memory_start: u32,
    pub extra_memory_end: u32,
    pub modem_support: String,
    pub memo: String,
    pub region: String,
}

impl MegaDriveROMHeader {
    pub fn new_from_file(path: &Path) -> Result<MegaDriveROMHeader, io::Error> {
        let mut f = File::open(path)?;

        // Get the header in one read
        f.seek(SeekFrom::Start(ROM_START as u64))?;

        let mut buf = [0; ROM_HEADER_SIZE as usize];

        f.read_exact(&mut buf)?;

        //TODO determine if the file is junk or a real rom.  The inclusion of any ASCII
        // spaces (0x20) would be a good heuristic (no spaces suggests it's probably junk)
        let system_name =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[0..16])).to_string();
        let copyright_notice =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[16..32])).to_string();
        let game_name_domestic =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[32..80])).to_string();
        let game_name_overseas =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[80..128])).to_string();
        let product_identifier =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[128..142])).to_string();

        // TODO Must be one-liner for this,
        // take the bytes and convert them from a big endian u16
        let mut t_checksum: [u8; 2] = [0; 2];
        t_checksum.clone_from_slice(&buf[142..144]);
        let checksum = u16::from_be_bytes(t_checksum);

        // TODO Could make this an Enum
        let device_support =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[144..160])).to_string();

        let mut t_rom_start: [u8; 4] = [0; 4];
        t_rom_start.clone_from_slice(&buf[160..164]);
        let rom_start = u32::from_be_bytes(t_rom_start);

        let mut t_rom_end: [u8; 4] = [0; 4];
        t_rom_end.clone_from_slice(&buf[164..168]);
        let rom_end = u32::from_be_bytes(t_rom_end);

        let mut t_ram_start: [u8; 4] = [0; 4];
        t_ram_start.clone_from_slice(&buf[168..172]);
        let ram_start = u32::from_be_bytes(t_ram_start);

        let mut t_ram_end: [u8; 4] = [0; 4];
        t_ram_end.clone_from_slice(&buf[172..176]);
        let ram_end = u32::from_be_bytes(t_ram_end);

        let t_extra_memory = String::from_utf8_lossy(&buf[176..178]).to_string();
        let extra_memory = t_extra_memory == "RA";

        let mut t_extra_memory_type: [u8; 1] = [0; 1];
        t_extra_memory_type.clone_from_slice(&buf[178..179]);
        let extra_memory_type = u8::from_be_bytes(t_extra_memory_type);

        let mut t_extra_memory_start: [u8; 4] = [0; 4];
        t_extra_memory_start.clone_from_slice(&buf[180..184]);
        let extra_memory_start = u32::from_be_bytes(t_extra_memory_start);

        let mut t_extra_memory_end: [u8; 4] = [0; 4];
        t_extra_memory_end.clone_from_slice(&buf[184..188]);
        let extra_memory_end = u32::from_be_bytes(t_extra_memory_end);

        let modem_support = String::from_utf8_lossy(&buf[188..200]).to_string();

        let memo =
            String::from_utf8_lossy(&shift_jis::shift_jis_to_utf8(&buf[200..240])).to_string();

        let region = String::from_utf8_lossy(&buf[240..243]).to_string();

        let a = MegaDriveROMHeader {
            system_name,
            copyright_notice,
            game_name_domestic,
            game_name_overseas,
            product_identifier,
            checksum,
            device_support,
            rom_start,
            rom_end,
            ram_start,
            ram_end,
            extra_memory,
            extra_memory_type,
            extra_memory_start,
            extra_memory_end,
            modem_support,
            memo,
            region,
        };

        //println!("Debug = {:#?}", a);
        Ok(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn megadrive_rom_header_from_file_the_spiral() {
        let a = MegaDriveROMHeader {
            system_name: "SEGA MEGA DRIVE ".to_string(),
            copyright_notice: "RESISTANCE 2019 ".to_string(),
            game_name_domestic: "THE SPIRAL, RELEASED AT EVOKE 2019 DEMO PARTY   ".to_string(),
            game_name_overseas: "THE SPIRAL: A MEGADRIVE DEMO BY RESISTANCE 2019 ".to_string(),
            product_identifier: "GM 00000000-00".to_string(),
            checksum: 0x6FE3,
            device_support: "JD              ".to_string(),
            rom_start: 0x00000000,
            rom_end: 0x003FFFFF,
            ram_start: 0x00FF0000,
            ram_end: 0x00FFFFFF,
            extra_memory: false,
            extra_memory_type: 0x00,
            extra_memory_start: 0x00200000,
            extra_memory_end: 0x002001FF,
            modem_support: " ".repeat(12),
            memo: "THE SPIRAL BY RESISTANCE                ".to_string(),
            region: "JU ".to_string(),
        };

        let path = Path::new("tests/bins/The_Spiral_by_Resistance_2019_60Hz.bin");

        let b = MegaDriveROMHeader::new_from_file(&path).unwrap();
        //assert_eq!(a, b);
        assert_eq!(a.system_name, b.system_name, "System Name not the same");
        assert_eq!(
            a.copyright_notice, b.copyright_notice,
            "Copyright notice not the same"
        );
        assert_eq!(
            a.game_name_domestic, b.game_name_domestic,
            "Game Name Domestic not the same"
        );
        assert_eq!(
            a.game_name_overseas, b.game_name_overseas,
            "Game Name overseas not the same"
        );
        assert_eq!(
            a.product_identifier, b.product_identifier,
            "Product identifier not the same"
        );
        assert_eq!(a.checksum, b.checksum, "Checksum not the same");
        assert_eq!(
            a.device_support, b.device_support,
            "Device support not the same"
        );
        assert_eq!(a.rom_start, b.rom_start, "ROM start not the same");
        assert_eq!(a.rom_end, b.rom_end, "ROM end not the same");
        assert_eq!(a.ram_start, b.ram_start, "RAM end not the same");
        assert_eq!(a.ram_end, b.ram_end, "RAM end not the same");
        assert_eq!(a.extra_memory, b.extra_memory, "Extra Memory not the same");
        assert_eq!(
            a.extra_memory_type, b.extra_memory_type,
            "Extra Memory Type not the same"
        );
        assert_eq!(
            a.extra_memory_start, b.extra_memory_start,
            "Extra Memory Start not the same"
        );
        assert_eq!(
            a.extra_memory_end, b.extra_memory_end,
            "Extra Memory End not the same"
        );
        assert_eq!(
            a.modem_support, b.modem_support,
            "Modem Support not the same"
        );
        assert_eq!(a.memo, b.memo, "Memo not the same");
        assert_eq!(a.region, b.region, "Region not the same");
    }

    #[test]
    fn megadrive_rom_header_from_file_xump2() {
        // The rom has extra memory (save ram)
        let path = Path::new("tests/bins/xump2_100-digital (free release).bin");

        let b = MegaDriveROMHeader::new_from_file(&path).unwrap();
        assert_eq!(true, b.extra_memory);
        assert_eq!(0x00, b.extra_memory_type);
        assert_eq!(0x00200001, b.extra_memory_start);
        assert_eq!(0x002001FF, b.extra_memory_end);
    }
}
