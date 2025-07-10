#![no_std]
extern crate alloc;

use crate::style::BATTERY_INDICATOR_STYLE;
use alloc::{fmt, format, string::ToString};
use chrono::{Date, DateTime, Utc};
use core::{fmt::Display, time::Duration};
use edgy::{
    embedded_graphics::{
        mono_font::ascii::{FONT_4X6, FONT_5X8},
        pixelcolor::BinaryColor,
        prelude::{DrawTarget, Size},
        text,
    },
    prelude::*,
    widgets::{
        battery::{Battery, BatteryStyle},
        grid_layout::GridLayoutBuilder,
        label::SevenSegmentStyleBuilder,
        linear_layout::LinearLayoutBuilder,
        margin_layout::MarginLayout,
    },
};

pub mod style;
pub mod widgets;
pub use chrono;

pub trait View<'a> {
    fn update<D: DrawTarget<Color = BinaryColor> + 'a>(&self) -> WidgetObject<'a, D, BinaryColor>;
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
pub struct DisplayPage {
    pub speed_km: u8,
    pub trip_km: u16,
    pub ride_time: Duration,
}

impl<'a> View<'a> for DisplayPage {
    fn update<D: DrawTarget<Color = BinaryColor> + 'a>(&self) -> WidgetObject<'a, D, BinaryColor> {
        let mut ui = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Center)
            .horizontal_alignment(LayoutAlignment::Center)
            .direction(LayoutDirection::Horizontal);

        ui.add_widget_obj(widgets::small_seven_segment_text(
            format!("{:0>4}", self.trip_km.clamp(0, 9999)),
            "TRIP km",
        ));

        let mut speed = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Center)
            .horizontal_alignment(LayoutAlignment::Center)
            .direction(LayoutDirection::Vertical);

        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(16, 32))
            .segment_width(4)
            .digit_spacing(4)
            .segment_color(BinaryColor::On)
            .build();

        speed.seven_segment(format!("{:0>2}", self.speed_km.clamp(0, 99)), style);
        speed.label("km/h", text::Alignment::Center, &FONT_5X8);

        ui.margin_layout(margin!(0, 6), |ui| {
            ui.add_widget_obj(speed.finish());
        });
        ui.add_widget_obj(widgets::very_small_seven_segment_text(
            format!("{}", FormatTime(self.ride_time)),
            "TIME",
        ));

        ui.finish()
    }
}

pub enum Page {
    Main,
    Log,
}

impl Default for Page {
    fn default() -> Self {
        Page::Main
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
                &FONT_5X8,
            );
        });
        margin_layout.add_widget_obj(main_grid.finish());
        margin_layout.finish()
    }
}
