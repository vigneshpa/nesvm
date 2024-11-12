use std::{error::Error, fmt::Display, io::Read};

use super::{m000::Mapper000, Mapper, NametableMirroring, System};

type Result<T> = core::result::Result<T, INesParseErr>;

#[derive(Debug)]
pub struct INesParseErr {
    message: &'static str,
}

impl INesParseErr {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}

impl Display for INesParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error ocoured while parsing iNES file: {}", self.message)
    }
}

impl Error for INesParseErr {}

fn validate_signature(file: &[u8]) -> Result<()> {
    let mesg = "Signature is not a valid UTF-8 string";
    // Validate the "NES" signature
    if file[0] != 0x4E || file[1] != 0x45 || file[2] != 0x53 || file[3] != 0x1A {
        Err(INesParseErr::new(mesg))
    } else {
        Ok(())
    }
}

pub fn parse(file: &[u8]) -> Result<Box<dyn Mapper>> {
    let is_nes_2 = ((file[7] & 0b0000_1100) >> 2) == 2;
    // let is_nes_2 = false;
    if is_nes_2 {
        parse_v2(file)
    } else {
        parse_v1(file)
    }
}

#[allow(unused)]
fn parse_v1(mut file: impl Read) -> Result<Box<dyn Mapper>> {
    let mut header = [0u8; 16];
    file.read_exact(&mut header).unwrap();

    validate_signature(&header)?;
    let n_prg_rom = header[4];
    let n_chr_rom = header[5];

    let flag6 = header[6];
    let nametable_mirroring = if flag6 & 0b0000_0001 == 0 {
        NametableMirroring::Horizontal
    } else {
        NametableMirroring::Vertical
    };
    let battery = flag6 & 0b0000_0010 != 0;
    let trainer = flag6 & 0b0000_0100 != 0;
    let alternative_nametable = flag6 & 0b0000_1000 != 0;
    let mapper_n = flag6 >> 4;

    // Reading triner
    let trainer = match trainer {
        true => {
            let mut data = [0u8; 512];
            file.read_exact(&mut data);
            // Moving into heap for faster movement across the program
            let data: Box<[u8; 512]> = Box::new(data);
            Some(data)
        }
        false => None,
    };

    drop(trainer);

    // Reading program ROM
    let mut pgr_rom = Vec::new();
    pgr_rom.resize(n_prg_rom as usize * 16384, 0);
    let mut chr_rom = Vec::new();
    chr_rom.resize(n_chr_rom as usize * 8192, 0);
    file.read_exact(pgr_rom.as_mut()).unwrap();
    file.read_exact(chr_rom.as_mut()).unwrap();

    // Constructing the mapper
    if mapper_n != 0 {
         ;
    }
    let mapper = match mapper_n {
        0 => Box::new(Mapper000::new(pgr_rom, chr_rom)),
        _ => return Err(INesParseErr::new("Unknown Mapper")),
    };
    Ok(mapper)
}

#[allow(unused)]
fn parse_v2(mut file: &[u8]) -> Result<Box<dyn Mapper>> {
    validate_signature(file)?;
    let mut n_prg_rom = file[4] as u16;
    let mut n_chr_rom = file[5] as u16;

    let flag6 = file[6];
    let nametable_mirroring = if flag6 & 0b0000_0001 == 0 {
        NametableMirroring::Horizontal
    } else {
        NametableMirroring::Vertical
    };
    let battery = flag6 & 0b0000_0010 != 0;
    let trainer = flag6 & 0b0000_0100 != 0;
    let alternative_nametable = flag6 & 0b0000_1000 != 0;
    let mut mapper_n = flag6 as u16 >> 4;

    let flag7 = file[7];
    let console_type = flag7 & 0b0000_0011;
    let nes_2_id = (flag7 & 0b0000_1100) >> 2;
    if nes_2_id != 2 {
        panic!("Not a valid NES2.0 file");
    }
    mapper_n |= (flag7 & 0b1111_0000) as u16;

    let flag8 = file[8];
    mapper_n |= ((flag8 & 0b0000_1111) as u16) << 8;
    let sub_mapper = flag8 & 0b1111_0000;

    let flag9 = file[9];
    n_prg_rom |= ((flag9 & 0b0000_1111) as u16) << 8;
    n_chr_rom |= ((flag9 & 0b1111_0000) as u16) << 4;

    let flag10 = file[10];
    let prg_ram = flag10 & 0b0000_1111;
    let prg_nvram = (flag10 & 0b1111_0000) >> 4;

    let flag11 = file[11];
    let chr_ram = flag11 & 0b0000_1111;
    let chr_nvram = (flag11 & 0b1111_0000) >> 4;

    let flag12 = file[12];
    let system = match flag12 & 0x03 {
        0 => System::NtscNES,
        1 => System::LicensedPalNES,
        2 => System::MultipleRegion,
        3 => System::Dendy,
        _ => unreachable!(),
    };

    let flag13 = file[13];
    let vs_ppu_type = flag13 & 0b0000_1111;
    let vs_hardware_type = (flag13 & 0b1111_0000) >> 4;

    let flag14 = file[14];
    let miscellaneous_roms = flag14 & 0x03;

    let flag15 = file[15];
    let default_expansion_device = flag15 & 0b0011_1111;

    // 16 bytes NES Header
    file = &file[16..];

    // Constructing the mapper
    let mapper = match mapper_n {
        0 => Box::new(Mapper000::new(Vec::new(), Vec::new())
        ),
        _ => return Err(INesParseErr::new("Unknown Mapper")),
    };
    Ok(mapper)
}
