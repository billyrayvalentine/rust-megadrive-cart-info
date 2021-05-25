# megadrive-cart-info
A commandline tool to inspect a Sega Mega Drive/Genesis rom image

# Example
```
$> megadrive-cart-info Virtua Racing (Europe).md

system name: SEGA MEGA DRIVE
copyright notice: (C)SEGA 1993.DEC
game name domestic: バーチャレーシング
game name overseas: Virtua Racing
product identifier: GM MK-1229 -00
checksum: 0x345D
device_support: J6
rom_start: 0x00000000
rom_end: 0x001FFFFF
ram_start: 0x00FF0000
ram_end: 0x00FFFFFF
extra_memory: false
extra_memory_type: 0x20
extra_memory_start: 0x20202020
extra_memory_end: 0x20202020
modem support:
memo: SV
region: E
```

# Features
* Partial implementation of shiftjis to UTF-8 (So far ASCII and most Katakana - as demonstrated above)
* No crate dependencies
* Pretty good test coverage

## Binaries for tests
The two 3rd party binaries in tests/bin used for unit testing.  They are not distrubuted in the RPM release
* Xump2 (freeware) by [Retro Guru](https://www.retroguru.com)
* [The Spiral by Resistance](https://www.resistance.no/?location=platforms&release=66)

# License
MIT

# About
* Written in Rust (author's first Rust project)
