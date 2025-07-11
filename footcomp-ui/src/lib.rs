#![no_std]
extern crate alloc;

use crate::{style::BATTERY_INDICATOR_STYLE, views::View};
use alloc::{fmt, format, string::ToString};
use chrono::{DateTime, Utc};
use core::{fmt::Display, time::Duration};
use edgy::{
    embedded_graphics::{
        mono_font::ascii::{FONT_4X6, FONT_5X7},
        pixelcolor::BinaryColor,
        prelude::{DrawTarget, Size},
        text,
    },
    prelude::*,
    widgets::{
        battery::{Battery, BatteryStyle},
        grid_layout::GridLayoutBuilder,
        margin_layout::MarginLayout,
    },
};

pub mod style;
pub mod widgets;
pub mod views;
pub use chrono;

pub enum Page {
    Main,
    Log,
}

impl Default for Page {
    fn default() -> Self {
        Page::Main
    }
}

pub struct FormatTime(pub Duration);
impl Display for FormatTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_secs = self.0.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        write!(f, "{:02}:{:02}:{:02}", hours, mins, secs)
    }
}

#[derive(Default)]
pub struct BaseUi {
    pub oat: i8,
    pub battery_percentage: u8,
    pub time: DateTime<Utc>,
}

impl BaseUi {
    pub fn update<'a, D: DrawTarget<Color = BinaryColor> + 'a>(
        &self,
        page: &impl View<'a>,
    ) -> WidgetObject<'a, D, BinaryColor> {
        let mut margin_layout = MarginLayout::new(margin!(3));

        let mut main_grid = GridLayoutBuilder::default()
            .add_row(12)
            .add_row(76)
            .add_row(12)
            .gap(2)
            .add_column(100);

        main_grid.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
            ui.label(
                self.time.format("%H:%M %A %d").to_string(),
                text::Alignment::Left,
                &FONT_4X6,
            );

            ui.horizontal_linear_layout(LayoutAlignment::End, |ui| {
                let bat_style =
                    BatteryStyle::new(BATTERY_INDICATOR_STYLE, LayoutDirection::Horizontal);
                ui.add_widget(Battery::new(
                    self.battery_percentage,
                    false,
                    Size::new(16, 6),
                    bat_style,
                ));
            });
        });

        main_grid.add_widget_obj(page.update());
        main_grid.horizontal_linear_layout(LayoutAlignment::Start, |ui| {
            ui.label(
                format!("OAT: {}C", self.oat),
                text::Alignment::Left,
                &FONT_5X7,
            );
        });
        margin_layout.add_widget_obj(main_grid.finish());
        margin_layout.finish()
    }
}
