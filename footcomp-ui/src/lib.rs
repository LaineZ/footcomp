#![no_std]
extern crate alloc;

use core::fmt::Debug;

use alloc::string::ToString;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
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

use crate::{style::BATTERY_INDICATOR_STYLE, widgets::tabbar::TabBarLabel};

pub mod style;
pub mod widgets;

pub fn main_ui<'a, D: DrawTarget<Color = BinaryColor> + 'a>() -> WidgetObject<'a, D, BinaryColor> {
    let mut ui = LinearLayoutBuilder::default()
        .vertical_alignment(LayoutAlignment::Center)
        .horizontal_alignment(LayoutAlignment::Center)
        .direction(LayoutDirection::Horizontal);

    ui.add_widget_obj(widgets::small_seven_segment_text("0000", "ODO km"));

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

    speed.seven_segment("50", style);
    speed.label("km/h", text::Alignment::Center, &FONT_5X8);

    ui.margin_layout(margin!(0, 6), |ui| {
        ui.add_widget_obj(speed.finish());
    });
    ui.add_widget_obj(widgets::small_seven_segment_text("00:00", "TIME"));

    ui.finish()
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

    main_grid.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
        ui.label(
            nt.format("%H:%M").to_string(),
            text::Alignment::Left,
            &FONT_4X6,
        );

        ui.horizontal_linear_layout(LayoutAlignment::Center, |ui| {
            ui.label("Tue Jul 2", text::Alignment::Center, &FONT_4X6);
        });

        ui.horizontal_linear_layout(LayoutAlignment::End, |ui| {
            let bat_style = BatteryStyle::new(BATTERY_INDICATOR_STYLE, LayoutDirection::Horizontal);
            ui.add_widget(Battery::new(30, false, Size::new(16, 6), bat_style));
        });
    });

    main_grid.add_widget_obj(main_ui());

    main_grid.horizontal_linear_layout(LayoutAlignment::Start, |ui| {
        ui.margin_layout(margin!(0, 3, 0, 0), |ui| {
            ui.add_widget(TabBarLabel::new(true, "DISPLAY"));
        });
        ui.add_widget(TabBarLabel::new(false, "LOG"));
    });

    margin_layout.add_widget_obj(main_grid.finish());

    margin_layout.finish()
}
