use super::{mystd::io::Read, File};
use alloc::vec::Vec;
use core::ops::Deref;

pub struct Mmap {
    vec: &'static [u8],
}

impl Mmap {
    pub unsafe fn map() -> Option<Mmap> {
        extern "C" {
            static HERMIT_ELF: [usize; 2];
        }

        let s = unsafe { super::mystd::slice::from_raw_parts(HERMIT_ELF[0] as _, HERMIT_ELF[1]) };

        // let mut mmap = Mmap {
        //     vec: Vec::with_capacity(len),
        // };
        // file.read_to_end(&mut mmap.vec).ok()?;
        Some(Mmap { vec: s })
    }
}

impl Deref for Mmap {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.vec[..]
    }
}
