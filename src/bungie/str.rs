use bytemuck::{Pod, Zeroable};
use std::str::{from_utf8, Utf8Error};

pub fn try_c_str(s: &[u8]) -> Result<&str, Utf8Error> {
    Ok(from_utf8(match memchr::memchr(0, s) {
        Some(n) => &s[..n],
        None => s,
    })?)
}

pub fn c_str(s: &[u8]) -> &str {
    try_c_str(s).expect("invalid utf8 string")
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CStrArray<const Size: usize>([u8; Size]);

unsafe impl<const Size: usize> Zeroable for CStrArray<Size> {}

unsafe impl<const Size: usize> Pod for CStrArray<Size> {}

impl<const Size: usize> std::fmt::Debug for CStrArray<Size> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.try_as_str() {
            Ok(s) => std::fmt::Debug::fmt(s, f),
            Err(_) => write!(f, "#error: invalid UTF8 {:?}", &self.0),
        }
    }
}

impl<const Size: usize> std::fmt::Display for CStrArray<Size> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.try_as_str() {
            Ok(s) => std::fmt::Display::fmt(s, f),
            Err(_) => write!(f, "#error: invalid UTF8 {:?}", &self.0),
        }
    }
}

impl<const Size: usize> CStrArray<Size> {
    pub fn as_bytes(&self) -> &[u8] {
        match memchr::memchr(0, &self.0) {
            Some(n) => &self.0[..n],
            None => &self.0,
        }
    }

    pub fn as_str(&self) -> &str {
        c_str(&self.0)
    }

    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        try_c_str(&self.0)
    }
}
