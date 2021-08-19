use std::os::raw::c_int;

use super::Font;
use crate::bind;

/// A text style of a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    /// A normal, plain style.
    Normal,
    /// A **bold** style.
    Bold,
    /// A *italic* style.
    Italic,
    /// An underline style.
    Underline,
    /// An ~~strike through~~ style.
    StrikeThrough,
}

impl FontStyle {
    fn from_raw(raw: c_int) -> Self {
        match raw as u32 {
            bind::TTF_STYLE_NORMAL => Self::Normal,
            bind::TTF_STYLE_BOLD => Self::Bold,
            bind::TTF_STYLE_ITALIC => Self::Italic,
            bind::TTF_STYLE_UNDERLINE => Self::Underline,
            bind::TTF_STYLE_STRIKETHROUGH => Self::StrikeThrough,
            _ => unreachable!(),
        }
    }

    fn into_raw(self) -> c_int {
        (match self {
            Self::Normal => bind::TTF_STYLE_NORMAL,
            Self::Bold => bind::TTF_STYLE_BOLD,
            Self::Italic => bind::TTF_STYLE_ITALIC,
            Self::Underline => bind::TTF_STYLE_UNDERLINE,
            Self::StrikeThrough => bind::TTF_STYLE_STRIKETHROUGH,
        }) as c_int
    }
}

/// An extension of [`FontStyle`] and outline width getters/setters.
pub trait StyleExt {
    /// Returns the current font style.
    fn font_style(&self) -> FontStyle;
    /// Sets the font style.
    fn set_font_style(&self, style: FontStyle);

    /// Returns the current outline width in pixels.
    fn outline_width(&self) -> u32;
    /// Sets the outline width in pixels.
    fn set_outline_width(&self, pixels: u32);
}

impl StyleExt for Font<'_> {
    fn font_style(&self) -> FontStyle {
        let raw = unsafe { bind::TTF_GetFontStyle(self.ptr.as_ptr()) };
        FontStyle::from_raw(raw)
    }

    fn set_font_style(&self, style: FontStyle) {
        // needed to check to prevent cache from erasing.
        if style != self.font_style() {
            unsafe { bind::TTF_SetFontStyle(self.ptr.as_ptr(), style.into_raw()) }
        }
    }

    fn outline_width(&self) -> u32 {
        unsafe { bind::TTF_GetFontOutline(self.ptr.as_ptr()) as _ }
    }

    fn set_outline_width(&self, pixels: u32) {
        if pixels != self.outline_width() {
            unsafe { bind::TTF_SetFontOutline(self.ptr.as_ptr(), pixels as _) }
        }
    }
}
