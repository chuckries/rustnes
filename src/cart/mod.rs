#![macro_escape]

use std::io::File;
use std::mem;

#[cfg(test)]
pub mod test;

pub type PrgRomBank = [u8, ..PRG_ROM_BANK_SIZE];
pub type PrgRom = Vec<PrgRomBank>;

static PRG_ROM_BANK_SIZE: uint = 0x4000; //16 KB
static CHR_ROM_BANK_SIZE: uint = 0x2000; //8 KB
static PRG_RAM_BANK_SIZE: uint = 0x2000; //8 KB
static TRAINER_SIZE: uint = 512;

/// # Header flags
///
/// Adapted from http://wiki.nesdev.com/w/index.php/INES
///
/// ## Flags 6
///
/// 76543210
/// ||||||||
/// ||||+||+- 0xx0: vertical arrangement/horizontal mirroring (CIRAM A10 = PPU A11)
/// |||| ||   0xx1: horizontal arrangement/vertical mirroring (CIRAM A10 = PPU A10)
/// |||| ||   1xxx: four-screen VRAM
/// |||| |+-- 1: SRAM in CPU $6000-$7FFF, if present, is battery backed
/// |||| +--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
/// ++++----- Lower nybble of mapper number
///
/// ## Flags 7
///
/// 76543210
/// ||||||||
/// |||||||+- VS Unisystem
/// ||||||+-- PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
/// ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
/// ++++----- Upper nybble of mapper number
///
/// ## Flags 9
///
/// 76543210
/// ||||||||
/// |||||||+- TV system (0: NTSC; 1: PAL)
/// +++++++-- Reserved, set to zero
///
/// ## Flags 10
///
/// 76543210
///   ||  ||
///   ||  ++- TV system (0: NTSC; 2: PAL; 1/3: dual compatible)
///   |+----- SRAM in CPU $6000-$7FFF is 0: present; 1: not present
///   +------ 0: Board has no bus conflicts; 1: Board has bus conflicts

#[packed]
struct CartHeader {
    identifier: [u8, ..4], // NES^
    pub prg_rom_count: u8, // in 16KB units
    pub chr_rom_count: u8, // in 8KB units
    flags_6: u8,
    flags_7: u8,
    pub prg_ram_count: u8, // in 8KB, minimum 8KB for compat
    flags_9: u8,
    flags_10: u8,
    pub zeros: [u8, ..5],
}

fn is_flag_set(flags: u8, flag: u8) -> bool {
    flags & flag != 0
}

impl CartHeader {
    pub fn new(bytes: &[u8, ..0x10]) -> Option<CartHeader> {
        let cart_header: &CartHeader;

        unsafe {
            cart_header = mem::transmute(bytes.as_ptr());
        }

        if cart_header.is_valid() {
            Some(*cart_header)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        static MSDOS_EOF: u8 = 0x1a;

        if self.identifier != ['N' as u8, 'E' as u8, 'S' as u8, MSDOS_EOF] { return false; }
        if self.zeros != [0u8, ..5] { return false; }

        true
    }

    pub fn has_trainer(&self) -> bool {
        is_flag_set(self.flags_6, 1 << 2)
    }

    pub fn has_sram(&self) -> bool {
        is_flag_set(self.flags_6, 1 << 1)
    }
}

pub struct Cart {
    header: CartHeader,
    prg_rom: Vec<[u8, ..PRG_ROM_BANK_SIZE]>,
    chr_rom: Vec<[u8, ..CHR_ROM_BANK_SIZE]>,

    //trainer not yet supported, mostly because I don't know what it is
    _trainer: [u8, ..TRAINER_SIZE],
}

impl Cart {
    pub fn new(rom: &Path) -> Cart {
        let mut file = File::open(rom).unwrap();

        //get the header info
        let mut buf = [0u8, ..0x10];
        file.read(buf);
        let header = CartHeader::new(&buf).expect("Bad header");

        //read the prg_rom
        let mut prg_rom = Vec::new();
        for _ in range(0, header.prg_rom_count) {
            let mut buf = [0u8, ..PRG_ROM_BANK_SIZE];
            file.read(buf);
            prg_rom.push(buf);
        }

        //read the chr_rom
        let mut chr_rom = Vec::new();
        for _ in range(0, header.chr_rom_count) {
            let mut buf = [0u8, ..CHR_ROM_BANK_SIZE];
            file.read(buf);
            chr_rom.push(buf);
        }

        //read trainer if present
        let mut trainer = [0u8, ..TRAINER_SIZE];
        if header.has_trainer() {
            file.read(trainer);
        }

        Cart{ 
            header: header,
            prg_rom: prg_rom,
            chr_rom: chr_rom,
            _trainer: trainer,
        }
    }

    pub fn read_from_lower_bank(&self, addr: u16) -> u8 {
        //TODO mapper support isn't planned for a long time, so just read from bank 0
        self.prg_rom[0][addr as uint]
    }

    pub fn read_from_upper_bank(&self, addr: u16) -> u8 {
        //TODO mapper support isn't planned for a long time, so just read from bank 1
        self.prg_rom[1][addr as uint]
    }
}
