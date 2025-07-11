use core::time::Duration;

use alloc::format;
use chrono::{DateTime, Utc};
use edgy::{
    embedded_graphics::{
        mono_font::ascii::{FONT_4X6, FONT_5X7},
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
    min_speed: u8,
    calories: u16,
    max_speed: u8,
    distance_km: u16,
    date: DateTime<Utc>,
    oat: i8,
}

#[derive(Default)]
pub struct LogPage {
    pub entry: LogEntry,
    pub current_log_entry: usize,
    pub max_logs: usize,
}

impl<'a> View<'a> for LogPage {
    fn update<D: DrawTarget<Color = BinaryColor> + 'a>(&self) -> WidgetObject<'a, D, BinaryColor> {
        // let mut ui = MarginLayout::new(margin!(2, 0, 0, 0));
        let mut main_ui = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Start)
            .horizontal_alignment(LayoutAlignment::Stretch)
            .gap(2)
            .direction(LayoutDirection::Vertical);

        let mut log_layout = LinearLayoutBuilder::default()
            .vertical_alignment(LayoutAlignment::Start)
            .horizontal_alignment(LayoutAlignment::Stretch)
            .direction(LayoutDirection::Vertical)
            .style(LOG_ENTRY_STYLE.into());

        log_layout.margin_layout(margin!(3), |ui| {
            ui.vertical_linear_layout(LayoutAlignment::Start, |ui| {
                ui.margin_layout(margin!(0, 0, 3, 0), |ui| {
                    ui.label(format!("{}", self.entry.date), Alignment::Left, &FONT_5X7);
                });
                ui.horizontal_linear_layout(LayoutAlignment::Stretch, |ui| {
                    ui.label(
                        format!(
                            "DST: {} km\nOAT: {}C\nCAL: {}",
                            self.entry.distance_km, self.entry.oat, self.entry.calories
                        ),
                        Alignment::Left,
                        &FONT_4X6,
                    );

                    ui.label(
                        format!(
                            "AVG SPD: {} km/h\nMAX SPD: {} km/h\nMIN SPD: {} km/h",
                            self.entry.avg_speed, self.entry.max_speed, self.entry.min_speed
                        ),
                        Alignment::Left,
                        &FONT_4X6,
                    );
                });
            });
        });

        main_ui.add_widget_obj(log_layout.finish());

        main_ui.horizontal_linear_layout(LayoutAlignment::Center, |ui| {
            ui.label("<- 1/90 ->", Alignment::Center, &FONT_4X6);
        });

        //ui.add_widget_obj(main_ui.finish());
        main_ui.finish()
    }
}
