use std::fmt::{Display, LowerHex, UpperHex};
use bytemuck::{Pod, Zeroable};
use binread::BinRead;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Pod, Zeroable, BinRead)]
pub struct FourCC(pub u32);

impl Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch0 = (self.0 >> 24) as u8 as char;
        let ch1 = (self.0 >> 16) as u8 as char;
        let ch2 = (self.0 >> 8) as u8 as char;
        let ch3 = (self.0 >> 0) as u8 as char;
        if f.alternate() {
            write!(f, "'{}', '{}', '{}', '{}'", ch0, ch1, ch2, ch3)
        } else {
            write!(f, "{}{}{}{}", ch0, ch1, ch2, ch3)
        }
    }
}

impl LowerHex for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl FourCC {
    pub const fn new(s: &str) -> FourCC {
        if s.len() != 4 {
            panic!("FourCC must be 4 character long")
        }
        let b: &[u8] = s.as_bytes();
        FourCC(((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | (b[3] as u32))
    }
}

