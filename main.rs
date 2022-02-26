use std::fs;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut rom = fs::read("BurgerTime.nes")?;
    let salt_address = 0xB64;
    let lives_address = 0xB75;
    rom[salt_address] = 128;
    rom[lives_address] = 255;
    
    let mut pos = 0;
    let mut buffer = File::create("PatchedBurgerTime.nes")?;
    while pos < rom.len() {
        let bytes_written = buffer.write(&rom[pos..])?;
        pos += bytes_written;
    }

    Ok(())
}