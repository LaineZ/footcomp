use alloc::string::String;
use edgy::{
    embedded_graphics::{
        mono_font::ascii::{FONT_4X6, FONT_5X8},
        pixelcolor::BinaryColor,
        prelude::*,
        text,
    },
    prelude::*,
    widgets::{
        label::{Label, LabelOptions, SevenSegmentStyleBuilder},
        linear_layout::LinearLayoutBuilder,
    },
};

pub mod tabbar;

pub fn small_seven_segment_text<'a, D: DrawTarget<Color = BinaryColor> + 'a, S: Into<String>>(
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
        LabelOptions::new().alignment(text::Alignment::Center),
        &FONT_5X8,
    ));
    layout.finish()
}

pub fn very_small_seven_segment_text<
    'a,
    D: DrawTarget<Color = BinaryColor> + 'a,
    S: Into<String>,
>(
    text: S,
    heading_text: &str,
) -> WidgetObject<'a, D, BinaryColor> {
    let style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(5, 16))
        .segment_width(1)
        .digit_spacing(1)
        .segment_color(BinaryColor::On)
        .build();

    let mut layout = LinearLayoutBuilder::default()
        .direction(LayoutDirection::Vertical)
        .horizontal_alignment(LayoutAlignment::Center);

    layout.seven_segment(text, style);
    layout.add_widget(Label::new(
        heading_text,
        LabelOptions::new().alignment(text::Alignment::Center),
        &FONT_5X8,
    ));
    layout.finish()
}
