#![macro_escape]

use std::io::File;
use std::mem;

use cpu::Cpu;

#[cfg(test)]
pub mod test;

static PRG_ROM_BANK_SIZE: uint = 0x4000; //16 KB
type PrgRomBank = [u8, ..PRG_ROM_BANK_SIZE];
pub type PrgRom = Vec<PrgRomBank>;

static CHR_ROM_BANK_SIZE: uint = 0x2000; //8 KB
type ChrRomBank = [u8, ..CHR_ROM_BANK_SIZE];
pub type ChrRom = Vec<ChrRomBank>;

//currently unused, not sure what it does
static PRG_RAM_BANK_SIZE: uint = 0x2000; //8 KB
type PrgRamBank = [u8, ..PRG_RAM_BANK_SIZE];
type PrgRam = Vec<PrgRamBank>;

//currently unused, not sure what it does
static TRAINER_SIZE: uint = 512;
type Trainer = [u8, ..TRAINER_SIZE];


pub struct Nes {
    rom_path: Path,

    //components
    cpu: Cpu,
}

impl Nes {
    pub fn new(rom_path: Path) -> Nes {
        info!("Rom Path: {}", rom_path.display());
        
        let (rom_header, prg_rom, chr_rom) = Nes::read_rom(&rom_path);

        //TODO Get things like horizontal/vertical scrolling here


        let cpu = Cpu::new(prg_rom);

        //hand back a Nes struct. At this point it only has ownership of the Cpu.
        //I'm still fairly unsure how this will look down the road
        //I will likely hide the PPU behind mem, but I will also want access to it here
        Nes { 
            rom_path: rom_path,

            cpu: cpu, 
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }

    fn read_rom(path: &Path) -> (RomHeader, PrgRom, ChrRom) {
        let mut file = File::open(path).unwrap();

        //get the header info
        let mut buf = [0u8, ..0x10];
        file.read(buf);
        let header = RomHeader::new(&buf).expect("Bad header");

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

        (header, prg_rom, chr_rom)
    }
}

/// # Header flags
///
/// from http://wiki.nesdev.com/w/index.php/INES
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
struct RomHeader {
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


impl RomHeader {
    pub fn new(bytes: &[u8, ..0x10]) -> Option<RomHeader> {
        let cart_header: &RomHeader;

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
        RomHeader::is_flag_set(self.flags_6, 1 << 2)
    }

    pub fn has_sram(&self) -> bool {
        RomHeader::is_flag_set(self.flags_6, 1 << 1)
    }

    fn is_flag_set(flags: u8, flag: u8) -> bool {
        flags & flag != 0
    }
}
