use std::mem;

use nes::{ChrRom, CHR_ROM_BANK_SIZE};
use nes::{VAddr};

#[cfg(test)]
pub mod test;

/// # Memory Map
/// This is from http://nesdev.com/NESDoc.pdf
///  ___________________ $10000  ________________
/// | Mirrors           |       | Mirrors        |
/// | $0000-$3FFF       |       | $0000-$3FFF    |
/// |___________________| $4000 |________________|
/// | Mirrors           |       |                |
/// | $3F00-$3F1F       |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $3F20 |                |
/// | Sprite Palette    |       | Palettes       |
/// |_ _ _ _ _ _ _ _ _ _| $3F10 |                |
/// | Image Palette     |       |                |
/// |___________________| $3F00 |________________|
/// | Mirrors           |       |                |
/// | $2000-$2EFF       |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $3000 |                |
/// | Attribute Table 3 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $2FC0 |                |
/// | Name Table 3      |       |                |
/// |___________________| $2C00 |                |
/// | Attribute Table 2 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $2BC0 |                |
/// | Name Table 2      |       | Name Tables    |
/// |___________________| $2800 |                |
/// | Attribute Table 1 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $27C0 |                |
/// | Name Table 1      |       |                |
/// |___________________| $2400 |                |
/// | Attribute Table 0 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $23C0 |                |
/// | Name Table 0      |       |                |
/// |___________________| $2000 |________________|
/// | Pattern Table 1   |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $1000 | Pattern Tables |
/// | Pattern Table 0   |       |                |
/// |___________________| $0000 |________________|
///
///
/// # PPU Registers
///
/// This is from http://nesdev.com/NESDoc.pdf
///
/// $2000 - PPU Control Register 1 - Write Only
///
/// - Bits 1-0 -  Name Table address, changes between the four
///   name tables at $2000 (0b00), $2400 (0b01), $2800 (0b10) and $2C00 (0b11).
///
/// - Bit 2 - Specifies the amout to increment address by, either 1 if 
///   this is 0 or 32 if this is 1.
///
/// - Bit 3 - Identifies which pattern table Sprites are stored in,
///   either $0000 (0) or $1000 (1).
///
/// - Bit 4 - Identifies which pattern table the background 
///   is store in, either $0000 (0) or $1000 (1).
///
/// - Bit 5 - Specifies the size of sprites in pixels, 8x8 
///   if this is 0, 8x16 if this is 1.
///
/// - Bit 6 - Changes PPU between master and slave modes. 
///   This is not used by the NES.
///
/// - Bit 7 - Indicates whether a NMI should occur up V-Blank.
///
/// $2001 - PPU Control Register 2 - Write Only
///
/// - Bit 0 - Indicates whether the system is in color (0) or 
///   monochrome mode (1).
///
/// - Bit 1 - Specifies whether to clip the background, that is whether
///   to hide the background in the left 8 pixels on screen (0) or 
///   or to show them (1).
///
/// - Bit 2 - Specifies whether to clip the sprites, that is whether
///   to hide sprites in the left 8 pixels on screen (0) or 
///   or to show them (1).
///
/// - Bit 3 - If this is 0, the background should not be displayed
///
/// - Bit 4 - If this is 0, sprites should not be dispalyed
///
/// - Bits 7-5 - Indicates background color in monochrome mode or 
///   color intensity in color mode.
///
/// $2002 - PPU Status Register - Read Only
///
/// - Bit 4 - If set, indicates that writes to VRAM should be ignored.
///
/// - Bit 5 - Scanline sprite count, if set, indicates more than 8 
///   sprites on the current scanline.
///
/// - Bit 6 - Sprite 0 hit flag, set when a non-transparent pixel of
///   sprite 0 overlaps a non-transparent background pixel
///
/// - Bit 7 - Indicates whether V-Blank is occurring.
///
/// $2003 - SprRam VAddr Register - Write Only
///
/// - Holds the VAddr in SprRam to access on the next write to $2004
///
/// $2004 - SprRam I/O Register - Write Only
///
/// - Writes a byte to SprRam at the VAddr indicated by $2003
///
/// $2005 - VRAM Address Register 1 - Write Only
///
/// $2006 - VRAM Address Register 2 - Write Only
///
/// $2007 - VRAM I/O Register - Read/Write
///
/// - Reads or writes a byte from VRAM at the current address.
///
/// TODO
/// DMA Register ($4014) and Joypad I/O Registers ($4016 and $4017)
///





/// # Sprites
///
/// This is from http://nesdev.com/NESDoc.pdf but
/// better doc can be found at http://wiki.nesdev.com/w/index.php/PPU_OAM
///
/// - Byte 0 - Stores the Y coordinate of the top of the sprite minus 1
/// - Byte 1 - Index number of the sprite in the patter tables
/// - Byte 2 - Stores the attributes of the sprites
/// -- Bits 1-0 - Most signifigant bits of the color
/// -- Bit 5    - Indicates whether this sprite has priority over the background
/// -- Bit 6    - Indicates whether to flip the sprite horizontally
/// -- Bit 7    - Indicates whether to flip the sprite vetically
/// - Byte 3 - Stores the X coordinate of the left of the sprite
/// -- X-scroll values of F9-FF do NOT result in the sprite wrapping 
///    around to the left side of the screen.

static SPR_RAM_SIZE: uint = 256;
type SprRamBuf = [u8, ..SPR_RAM_SIZE];

struct SprRam {
    buf: SprRamBuf,
}

impl Index<u16, u8> for SprRam {
    #[inline]
    fn index<'a>(&'a self, index: &u16) -> &'a u8 {
        &self.buf[*index as uint]
    }
}

impl SprRam {
    pub fn new() -> SprRam {
        SprRam {
            buf: [0u8, ..SPR_RAM_SIZE],
        }
    }

    #[inline]
    pub fn spr<'a>(&'a self, idx: uint) -> Spr<'a> {
        Spr {
            spr: self.buf.as_slice().slice_from(idx << 2),
        }
    }
}

static SPR_COLOR_MASK: u8    = 0b00000011;
static SPR_PRIORITY_FLAG: u8 = 0b00100000;
static SPR_H_FLIP: u8        = 0b01000000;
static SPR_V_FLIP: u8        = 0b10000000;

struct Spr<'a> {
    spr: &'a[u8],
}

impl<'a> Spr<'a> {
    #[inline]
    pub fn y(&self) -> u8 {
        self.spr[0]
    }

    #[inline]
    pub fn x(&self) -> u8 {
        self.spr[3]
    }

    #[inline]
    pub fn idx(&self) -> u8 {
        self.spr[1]
    }

    #[inline]
    pub fn color(&self) -> u8 {
        (self.spr[2] & SPR_COLOR_MASK) << 2
    }

    #[inline]
    pub fn has_priority(&self) -> bool {
        (self.spr[2] & SPR_PRIORITY_FLAG) > 0
    }

    //TODO I might night actually need access to the flip attributes outside of Spr if I return
    //iterators over they bytes that I actually want to draw. I could do all the v_flip/h_flip
    //internally
    pub fn h_flip(&self) -> bool {
        (self.spr[2] & SPR_H_FLIP) > 0
    }

    pub fn v_flip(&self) -> bool {
        (self.spr[2] & SPR_V_FLIP) > 0
    }
}

static PATTERN_TABLE_SIZE: uint = 0x1000;
static NAME_TABLE_SIZE: uint = 0x03C0;
static ATTRIBUTE_TABLE_SIZE: uint = 0x0040;

struct VRam {
    buf: [u8, ..0x2400], //enough for the pattern tables and one name table/attribute table
}

impl Index<u16, u8> for VRam {
    fn index<'a>(&'a self, index: &u16) -> &'a u8 {
        &self.buf[*index as uint]
    }
}

impl IndexMut<u16, u8> for VRam {
    fn index_mut<'a>(&'a mut self, index: &u16) -> &'a mut u8 {
        &mut self.buf[*index as uint]
    }
}

impl VRam {
    pub fn new(chr_rom: ChrRom) -> VRam {
        let mut vram_bytes = [0u8, ..0x2400];

        //TODO full ChrRomBank support
        for i in range(0u, chr_rom[0].len()) {
            vram_bytes[i] = chr_rom[0][i];
        }

        VRam {
            buf: vram_bytes,
        }
    }

    pub fn pattern_table<'a>(&'a self, idx: uint) -> PatternTable<'a> {
        let pattern_table = 
            if idx % 2 == 0 { self.buf.as_slice().slice_to(PATTERN_TABLE_SIZE) }
            else {
                self.buf.as_slice().slice_from(PATTERN_TABLE_SIZE).slice_to(PATTERN_TABLE_SIZE)
            };

        PatternTable { 
            pattern_table: pattern_table,
        }
    }

    pub fn name_table<'a>(&'a self, idx: uint) -> NameTable<'a> {
        let name_table = self.buf.as_slice().slice_from(0x2000).slice_to(NAME_TABLE_SIZE + ATTRIBUTE_TABLE_SIZE);

        NameTable {
            name_table: name_table,
        }
    }
}

struct PatternTable<'a> {
    pattern_table: &'a[u8],
}

impl<'a> Index<u8, u8> for PatternTable<'a> {
    fn index<'a>(&'a self, index: &u8) -> &'a u8 {
        &self.pattern_table[*index as uint]
    }
}

struct NameTable<'a> {
    name_table: &'a[u8],
}

impl<'a> NameTable<'a> {
    fn attr_table<'a>(&'a self) -> AttrTable<'a> {
        let attr_table = self.name_table.slice_from(NAME_TABLE_SIZE).slice_to(ATTRIBUTE_TABLE_SIZE);

        AttrTable {
            attr_table: attr_table,
        }
    }
}

struct AttrTable<'a> {
    attr_table: &'a[u8],
}

























struct PpuRegisters {
    ppu_status: PpuStatus,
}

impl PpuRegisters {
    pub fn new() -> PpuRegisters {
        PpuRegisters {
            ppu_status: PpuStatus::new(),
        }
    }
}

//TODO least significant bits
struct PpuStatus {
    sprite_overflow: bool,
    sprite_zero_hit: bool,
    v_blank: bool,
}

impl PpuStatus {
    pub fn new() -> PpuStatus {
        PpuStatus {
            sprite_overflow: false,
            sprite_zero_hit: false,
            v_blank: false,
        }
    }
    pub fn read(&self) -> u8 {
        let mut reg: u8 = 0;
        if self.sprite_overflow { reg |= 0b00100000; }
        if self.sprite_zero_hit { reg |= 0b01000000; }
        if self.v_blank { reg |= 0b10000000; }
        reg
    }
}

pub struct Ppu {
    vram: VRam,
    spr_ram: SprRam,
    registers: PpuRegisters,
}

impl Ppu {
    pub fn new(chr_rom: ChrRom) -> Ppu {
        let vram = VRam::new(chr_rom);
        let spr_ram = SprRam::new();

        Ppu {
            vram: vram,
            spr_ram: spr_ram,
            registers: PpuRegisters::new(),
        }
    }

    //$2002
    pub fn read_ppu_status(&mut self) -> u8 {
        let reg = self.registers.ppu_status.read();
        self.registers.ppu_status.v_blank = false;

        reg
    }

    pub fn do_scanline(&mut self, scanline: uint) {
        info!("Scanline: {}", scanline);
        if scanline == 240 { self.registers.ppu_status.v_blank = true; }
        if scanline == 260 { 
            self.registers.ppu_status.v_blank = false;
            self.registers.ppu_status.sprite_zero_hit = false;
            self.registers.ppu_status.sprite_overflow = false;
        }
    }

/// # Memory Map
/// This is from http://nesdev.com/NESDoc.pdf
///  ___________________ $10000  ________________
/// | Mirrors           |       | Mirrors        |
/// | $0000-$3FFF       |       | $0000-$3FFF    |
/// |___________________| $4000 |________________|
/// | Mirrors           |       |                |
/// | $3F00-$3F1F       |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $3F20 |                |
/// | Sprite Palette    |       | Palettes       |
/// |_ _ _ _ _ _ _ _ _ _| $3F10 |                |
/// | Image Palette     |       |                |
/// |___________________| $3F00 |________________|
/// | Mirrors           |       |                |
/// | $2000-$2EFF       |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $3000 |                |
/// | Attribute Table 3 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $2FC0 |                |
/// | Name Table 3      |       |                |
/// |___________________| $2C00 |                |
/// | Attribute Table 2 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $2BC0 |                |
/// | Name Table 2      |       | Name Tables    |
/// |___________________| $2800 |                |
/// | Attribute Table 1 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $27C0 |                |
/// | Name Table 1      |       |                |
/// |___________________| $2400 |                |
/// | Attribute Table 0 |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $23C0 |                |
/// | Name Table 0      |       |                |
/// |___________________| $2000 |________________|
/// | Pattern Table 1   |       |                |
/// |_ _ _ _ _ _ _ _ _ _| $1000 | Pattern Tables |
/// | Pattern Table 0   |       |                |
/// |___________________| $0000 |________________|
    pub fn read_byte(&self, virtual_address: VAddr) -> u8 {
        if virtual_address < 0x2000 {
            self.vram[virtual_address]
        } else {
            0
        }
    }

    pub fn write_byte(&mut self, virtual_address: VAddr, val: u8) {
        if virtual_address < 0x2000 {
            self.vram[virtual_address] = val;
        }
    }
}





















type rgb = [u8, ..3];
static SYSTEM_PALETTE_SIZE: uint = 0x40;
static SYSTEM_PALETTE: [rgb, ..SYSTEM_PALETTE_SIZE] = [
    [0x75, 0x75, 0x75], //00
    [0x27, 0x1B, 0x8F], //01
    [0x00, 0x00, 0xAB], //02
    [0x47, 0x00, 0x9F], //03
    [0x8F, 0x00, 0x77], //04
    [0xAB, 0x00, 0x13], //05
    [0xA7, 0x00, 0x00], //06
    [0x7F, 0x0B, 0x00], //07
    [0x43, 0x2F, 0x00], //08
    [0x00, 0x47, 0x00], //09
    [0x00, 0x51, 0x00], //0A
    [0x00, 0x3F, 0x17], //0B
    [0x1B, 0x3F, 0x5F], //0C
    [0x00, 0x00, 0x00], //0D
    [0x00, 0x00, 0x00], //0E
    [0x00, 0x00, 0x00], //0F
    [0xBC, 0xBC, 0xBC], //10
    [0x00, 0x73, 0xEF], //11
    [0x23, 0x3B, 0xEF], //12
    [0x83, 0x00, 0xF3], //13
    [0xBF, 0x00, 0xBF], //14
    [0xE7, 0x00, 0x5B], //15
    [0xDB, 0x2B, 0x00], //16
    [0xCB, 0x4F, 0x0F], //17
    [0x8B, 0x73, 0x00], //18
    [0x00, 0x97, 0x00], //19
    [0x00, 0xAB, 0x00], //1A
    [0x00, 0x93, 0x3B], //1B
    [0x00, 0x83, 0x8B], //1C
    [0x00, 0x00, 0x00], //1D
    [0x00, 0x00, 0x00], //1E
    [0x00, 0x00, 0x00], //1F
    [0xFF, 0xFF, 0xFF], //20
    [0x3F, 0xBF, 0xFF], //21
    [0x5F, 0x97, 0xFF], //22
    [0xA7, 0x8B, 0xFD], //23
    [0xF7, 0x7B, 0xFF], //24
    [0xFF, 0x77, 0xB7], //25
    [0xFF, 0x77, 0x63], //26
    [0xFF, 0x9B, 0x3B], //27
    [0xF3, 0xBF, 0x3F], //28
    [0x83, 0xD3, 0x13], //29
    [0x4F, 0xDF, 0x4B], //2A
    [0x58, 0xF8, 0x98], //2B
    [0x00, 0xEB, 0xDB], //2C
    [0x00, 0x00, 0x00], //2D
    [0x00, 0x00, 0x00], //2E
    [0x00, 0x00, 0x00], //2F
    [0xFF, 0xFF, 0xFF], //30
    [0xAB, 0xE7, 0xFF], //31
    [0xC7, 0xD7, 0xFF], //32
    [0xD7, 0xCB, 0xFF], //33
    [0xFF, 0xC7, 0xFF], //34
    [0xFF, 0xC7, 0xDB], //35
    [0xFF, 0xBF, 0xB3], //36
    [0xFF, 0xDB, 0xAB], //37
    [0xFF, 0xE7, 0xA3], //38
    [0xE3, 0xFF, 0xA3], //39
    [0xAB, 0xF3, 0xBF], //3A
    [0xB3, 0xFF, 0xCF], //3B
    [0x9F, 0xFF, 0xF3], //3C
    [0x00, 0x00, 0x00], //3D
    [0x00, 0x00, 0x00], //3E
    [0x00, 0x00, 0x00], //3F
];
