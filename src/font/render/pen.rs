//! Extensions for rich-sdl2-rust.

use rich_sdl2_rust::{
    geo::Rect,
    renderer::{pen::Pen, PasteExt},
    texture::Texture,
};

use super::{RenderExt, RenderMode};
use crate::font::Font;

/// An extension for [`Pen`] to render a text.
pub trait FontRenderExt {
    /// Renders a text to the area with the font.
    fn text(&self, font: &Font, text: &str, mode: RenderMode, area: Option<Rect>);
}

impl FontRenderExt for Pen<'_> {
    fn text(&self, font: &Font, text: &str, mode: RenderMode, area: Option<Rect>) {
        let surface = font.render(text, mode).expect("rendering text failed");
        let texture = Texture::from_surface(self.renderer(), &surface);
        self.renderer().paste(texture, area);
    }
}
