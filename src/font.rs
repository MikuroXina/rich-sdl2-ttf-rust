//! Font data managers.

use rich_sdl2_rust::{Result, Sdl, SdlError};
use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use crate::{bind, Ttf};

pub use attribute::*;
pub use metric::*;
pub use setting::*;
pub use style::*;

use self::glyph::Glyph;

mod attribute;
pub mod glyph;
mod metric;
mod setting;
mod style;

/// A font data structure.
pub struct Font<'ttf> {
    ptr: NonNull<bind::TTF_Font>,
    _phantom: PhantomData<&'ttf Ttf>,
}

impl<'ttf> Font<'ttf> {
    /// Constructs a font data from file name and point size.
    /// The font face in the font file can be selected by an optional index, which will be `0` if `None`.
    ///
    /// # Panics
    ///
    /// Panics if `file_name` is empty.
    pub fn new(_ttf: &'ttf Ttf, file_name: &str, point: u32, index: Option<usize>) -> Result<Self> {
        let file_name_cstr = CString::new(file_name).expect("file name must not be empty");
        let ptr = unsafe {
            bind::TTF_OpenFontIndex(file_name_cstr.as_ptr(), point as _, index.unwrap_or(0) as _)
        };
        if ptr.is_null() {
            Err(SdlError::Others { msg: Sdl::error() })
        } else {
            Ok(Self {
                ptr: NonNull::new(ptr).unwrap(),
                _phantom: PhantomData,
            })
        }
    }

    /// Returns the glyph of the font if exists.
    pub fn glyph(&self, ch: char) -> Option<Glyph> {
        Glyph::new(self, ch)
    }
}

impl Drop for Font<'_> {
    fn drop(&mut self) {
        unsafe { bind::TTF_CloseFont(self.ptr.as_ptr()) }
    }
}
