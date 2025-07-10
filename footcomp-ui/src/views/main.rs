use core::time::Duration;

use alloc::format;
use edgy::{
    embedded_graphics::{
        mono_font::ascii::FONT_5X8,
        pixelcolor::BinaryColor,
        prelude::{DrawTarget, Size},
        text,
    },
    prelude::*,
    widgets::{
        label::SevenSegmentStyleBuilder,
        linear_layout::LinearLayoutBuilder,
    },
};

use crate::{views::View, widgets, FormatTime};

#[derive(Default)]
pub struct MainPage {
    pub speed_km: u8,
    pub trip_km: u16,
    pub ride_time: Duration,
}

impl<'a> View<'a> for MainPage {
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