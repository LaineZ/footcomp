use edgy::{embedded_graphics::{pixelcolor::BinaryColor, prelude::DrawTarget}, widgets::WidgetObject};

pub mod main;
pub mod log;

pub trait View<'a> {
    fn update<D: DrawTarget<Color = BinaryColor> + 'a>(&self) -> WidgetObject<'a, D, BinaryColor>;
}
