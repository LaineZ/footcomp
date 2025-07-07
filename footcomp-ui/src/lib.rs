use edgy::{
    embedded_graphics::{
        mono_font::ascii::{FONT_4X6, FONT_5X7, FONT_5X8, FONT_8X13_BOLD},
        pixelcolor::BinaryColor,
        prelude::{DrawTarget, Size},
        text,
    },
    prelude::*,
    widgets::{
        button::{Button, ButtonGeneric},
        grid_layout::GridLayoutBuilder,
        label::{Label, LabelOptions, SevenSegmentStyleBuilder},
        linear_layout::LinearLayoutBuilder,
        margin_layout::MarginLayout, toggle_button::ToggleButton,
    },
};

use crate::style::BUTTON_STYLE;

pub mod style;

fn small_seven_segment_text<'a, D: DrawTarget<Color = BinaryColor> + 'a, S: Into<String>>(
    text: S,
    heading_text: &str,
) -> WidgetObject<'a, D, BinaryColor> {
    let style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(6, 10))
        .segment_width(1)
        .digit_spacing(1)
        .segment_color(BinaryColor::On)
        .build();

    let mut layout = LinearLayoutBuilder::default().direction(LayoutDirection::Horizontal);

    layout.horizontal_linear_layout(LayoutAlignment::Start, |ui| {
        ui.add_widget(Label::new(
            heading_text,
            LabelOptions::new().line_height(0),
            &FONT_5X8,
        ));
        ui.margin_layout(margin!(0, 0, 0, 5), |ui| {
            ui.seven_segment(text, style);
        });
    });

    layout.finish()
}

pub fn main_ui<'a, D: DrawTarget<Color = BinaryColor> + 'a>() -> WidgetObject<'a, D, BinaryColor> {
    let mut ui = LinearLayoutBuilder::default()
        .alignment(LayoutAlignment::Start)
        .direction(LayoutDirection::Horizontal);

    ui.margin_layout(margin!(0, 5, 0, 0), |ui| {
        ui.horizontal_linear_layout(LayoutAlignment::Start, |speed| {
            let style = SevenSegmentStyleBuilder::new()
                .digit_size(Size::new(12, 40))
                .segment_width(4)
                .digit_spacing(4)
                .segment_color(BinaryColor::On)
                .build();

            speed.seven_segment("50", style);
            speed.margin_layout(margin!(0, 0, 0, 5), |ui| {
                ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
                    ui.add_widget(Label::new(
                        "km/h",
                        LabelOptions::new().line_height(0),
                        &FONT_5X8,
                    ));
                });
            });
        });
    });

    ui.vertical_linear_layout(LayoutAlignment::Start, |odo| {
        odo.add_widget_obj(small_seven_segment_text("0000", "ODO "));
        odo.add_widget_obj(small_seven_segment_text("00:00", "TIME"));
        odo.add_widget_obj(small_seven_segment_text("0", "TRIP"));
    });

    ui.finish()
}

pub fn base_ui<'a, D: DrawTarget<Color = BinaryColor> + 'a>() -> WidgetObject<'a, D, BinaryColor> {
    let mut margin_layout = MarginLayout::new(margin!(1));

    let mut main_grid = GridLayoutBuilder::default()
        .add_row(80)
        .add_row(20)
        .add_column(100);

    main_grid.add_widget_obj(main_ui());

    main_grid.horizontal_linear_layout(LayoutAlignment::Start, |ui| {
        ui.add_widget(ToggleButton::new_styled(
            "DISPLAY".to_string(),
            ButtonGeneric::new(&FONT_4X6, text::Alignment::Left, BUTTON_STYLE, 0),
            true,
            Box::new(|bool| {}),
        ));
    });

    margin_layout.add_widget_obj(main_grid.finish());

    margin_layout.finish()
}
