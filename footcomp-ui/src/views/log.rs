use core::time::Duration;

use chrono::{DateTime, Utc};
use edgy::{
    embedded_graphics::{
        mono_font::ascii::{FONT_4X6, FONT_5X8},
        pixelcolor::BinaryColor,
        prelude::DrawTarget,
        primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
        text::Alignment,
    },
    prelude::*,
    themes::WidgetStyle,
    widgets::{linear_layout::LinearLayoutBuilder, margin_layout::MarginLayout},
};

use crate::views::View;

const LOG_ENTRY_STYLE: PrimitiveStyle<BinaryColor> = PrimitiveStyleBuilder::new()
    .stroke_color(BinaryColor::On)
    .stroke_width(1)
    .build();

#[derive(Default)]
pub struct LogEntry {
    avg_speed: u8,
    distance_km: u16,
    date: DateTime<Utc>,
    oat: i8,
}

#[derive(Default)]
pub struct LogPage {
    pub entires: [LogEntry; 3],
    pub page: usize,
    pub max: usize,
}

impl<'a> View<'a> for LogPage {
    fn update<D: DrawTarget<Color = BinaryColor> + 'a>(&self) -> WidgetObject<'a, D, BinaryColor> {
        let mut ui = MarginLayout::new(margin!(2, 0, 0, 0));
        let mut main_ui = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Start)
            .horizontal_alignment(LayoutAlignment::Stretch)
            .gap(2)
            .direction(LayoutDirection::Vertical);

        for i in 0..2 {
            let mut log_layout = LinearLayoutBuilder::default()
                .alignment(LayoutAlignment::Start)
                .direction(LayoutDirection::Vertical)
                .style(LOG_ENTRY_STYLE.into());

            log_layout.margin_layout(margin!(3), |ui| {
                ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
                    ui.label("#1 10-01-2025 11:11Z", Alignment::Left, &FONT_5X8);
                    ui.label("DIST: 2 km | AVG SPD: 25 km/h", Alignment::Left, &FONT_4X6);
                });
            });

            main_ui.add_widget_obj(log_layout.finish());
        }

        ui.add_widget_obj(main_ui.finish());
        ui.finish()
    }
}
