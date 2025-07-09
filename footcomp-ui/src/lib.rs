#![no_std]

extern crate alloc;

use alloc::string::String;
use edgy::{
    embedded_graphics::{
        mono_font::ascii::FONT_5X8,
        pixelcolor::BinaryColor,
        prelude::{DrawTarget, Size},
        text,
    },
    prelude::*,
    widgets::{
        grid_layout::GridLayoutBuilder,
        label::{Label, LabelOptions, SevenSegmentStyleBuilder},
        linear_layout::LinearLayoutBuilder,
        margin_layout::MarginLayout,
    },
};

use crate::widgets::tabbar::TabBarLabel;

pub mod style;
pub mod widgets;

fn small_seven_segment_text<'a, D: DrawTarget<Color = BinaryColor> + 'a, S: Into<String>>(
    text: S,
    heading_text: &str,
) -> WidgetObject<'a, D, BinaryColor> {
    let style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(8, 16))
        .segment_width(2)
        .digit_spacing(1)
        .segment_color(BinaryColor::On)
        .build();

    let mut layout = LinearLayoutBuilder::default()
        .direction(LayoutDirection::Vertical)
        .horizontal_alignment(LayoutAlignment::Center);

    layout.seven_segment(text, style);
    layout.add_widget(Label::new(
        heading_text,
        LabelOptions::new()
            .alignment(text::Alignment::Center)
            .line_height(0),
        &FONT_5X8,
    ));
    layout.finish()
}

pub fn main_ui<'a, D: DrawTarget<Color = BinaryColor> + 'a>() -> WidgetObject<'a, D, BinaryColor> {
    let mut ui = LinearLayoutBuilder::default()
        .vertical_alignment(LayoutAlignment::Center)
        .horizontal_alignment(LayoutAlignment::Center)
        .direction(LayoutDirection::Horizontal);

    ui.add_widget_obj(small_seven_segment_text("0000", "ODO KM"));

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

    ui.margin_layout(margin!(0, 5), |ui| {
        ui.add_widget_obj(speed.finish());
    });
    ui.add_widget_obj(small_seven_segment_text("00:00", "TIME"));

    ui.finish()
}

pub fn base_ui<'a, D: DrawTarget<Color = BinaryColor> + 'a>() -> WidgetObject<'a, D, BinaryColor> {
    let mut margin_layout = MarginLayout::new(margin!(3));

    let mut main_grid = GridLayoutBuilder::default()
        .add_row(88)
        .add_row(12)
        .add_column(100);

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
