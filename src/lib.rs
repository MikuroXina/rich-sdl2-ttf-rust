//! # rich-sdl2-ttf-rust
//!
//! The rich-sdl2-ttf-rust provides wrapper for SDL2_ttf and abstractions of font rendering APIs.

#![warn(missing_docs)]

use rich_sdl2_rust::Sdl;
use static_assertions::assert_not_impl_all;
use std::{cell::Cell, marker::PhantomData};

mod bind;

/// A root SDL2_ttf controller.
#[derive(Debug)]
pub struct Ttf {
    _phantom: PhantomData<Cell<u8>>,
}

assert_not_impl_all!(Ttf: Send, Sync);

impl Ttf {
    /// Constructs a root controller.
    pub fn new() -> Self {
        let ret = unsafe { bind::TTF_Init() };
        if ret == -1 {
            Sdl::error_then_panic("Ttf");
        }
        Self {
            _phantom: PhantomData,
        }
    }
}

impl Drop for Ttf {
    fn drop(&mut self) {
        unsafe { bind::TTF_Quit() }
    }
}

impl Default for Ttf {
    fn default() -> Self {
        Self::new()
    }
}
