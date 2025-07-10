#![no_std]
extern crate alloc;

use core::{fmt::Display, time::Duration};

use alloc::{fmt, format, string::ToString};
use chrono::DateTime;
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

use crate::{style::BATTERY_INDICATOR_STYLE};

pub mod style;
pub mod widgets;

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
struct DisplayPage {
    pub speed: u8,
    pub trip_km: u16,
    pub ride_time: Duration,
}

impl DisplayPage {
    fn draw<'a, D: DrawTarget<Color = BinaryColor> + 'a>(&self) -> WidgetObject<'a, D, BinaryColor> {
        let mut ui = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Center)
            .horizontal_alignment(LayoutAlignment::Center)
            .direction(LayoutDirection::Horizontal);

        ui.add_widget_obj(widgets::small_seven_segment_text(format!("{:0>4}", self.trip_km), "TRIP km"));

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

        speed.seven_segment(format!("{:0>2}", self.speed), style);
        speed.label("km/h", text::Alignment::Center, &FONT_5X8);

        ui.margin_layout(margin!(0, 6), |ui| {
            ui.add_widget_obj(speed.finish());
        });
        ui.add_widget_obj(widgets::very_small_seven_segment_text(format!("{}", FormatTime(self.ride_time)), "TIME"));

        ui.finish()
    }
}

pub fn base_ui<'a, D: DrawTarget<Color = BinaryColor> + 'a>() -> WidgetObject<'a, D, BinaryColor> {
    let mut margin_layout = MarginLayout::new(margin!(3));

    let mut main_grid = GridLayoutBuilder::default()
        .add_row(12)
        .add_row(76)
        .add_row(12)
        .add_column(100);

    let ts: i64 = 1_720_000_000;
    let nt = DateTime::from_timestamp(ts, 0).unwrap();

    let mut display_page = DisplayPage::default();

    display_page.speed = 2;
    display_page.trip_km = 401;
    display_page.ride_time = Duration::from_secs(5940 + 59);

    main_grid.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
        ui.label(
            nt.format("%H:%M Jul 8").to_string(),
            text::Alignment::Left,
            &FONT_4X6,
        );

        ui.horizontal_linear_layout(LayoutAlignment::End, |ui| {
            let bat_style = BatteryStyle::new(BATTERY_INDICATOR_STYLE, LayoutDirection::Horizontal);
            ui.add_widget(Battery::new(30, false, Size::new(16, 6), bat_style));
        });
    });

    main_grid.add_widget_obj(display_page.draw());

    main_grid.horizontal_linear_layout(LayoutAlignment::Start, |ui| {
        ui.label("OAT: 28C", text::Alignment::Left, &FONT_5X8);
    });

    margin_layout.add_widget_obj(main_grid.finish());

    margin_layout.finish()
}
