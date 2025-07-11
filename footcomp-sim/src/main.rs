use std::time::{Duration, Instant, SystemTime};

use edgy::embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use edgy::{
    UiContext,
    themes::{self},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window, sdl2::Keycode};
use footcomp_ui::chrono::DateTime;
use footcomp_ui::views::log::{self, LogPage};
use footcomp_ui::views::main::MainPage;

fn main() -> Result<(), core::convert::Infallible> {
    let display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(4)
        .build();

    let mut window = Window::new("a bit edgy ui", &output_settings);
    let mut ui_ctx = UiContext::new(display, themes::hope_diamond::apply());
    let mut base_ui = footcomp_ui::BaseUi::default();
    let mut main_view = MainPage::default();
    let mut log_view = LogPage::default();

    let mut now = Instant::now();
    loop {
        let system_now = SystemTime::now();
        let timestamp = system_now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        base_ui.time = DateTime::from_timestamp(timestamp as i64, 0).unwrap();

        if now.elapsed() > Duration::from_millis(200) {
            main_view.speed_km += 1;
            base_ui.oat += 1;

            main_view.ride_time += now.elapsed();
            now = Instant::now();
        }
        main_view.speed_km %= 60;
        base_ui.oat %= 30;
        //base_ui.time = DateTime::now();

        window.update(&ui_ctx.draw_target);

        for event in window.events() {
            match event {
                embedded_graphics_simulator::SimulatorEvent::Quit => {
                    std::process::exit(0);
                }
                embedded_graphics_simulator::SimulatorEvent::KeyDown {
                    keycode,
                    keymod: _,
                    repeat: _,
                } => {
                    if keycode == Keycode::F1 {
                        ui_ctx.toggle_debug_mode();
                    }
                }
                _ => {}
            }
        }

        ui_ctx.draw_target.clear(BinaryColor::Off)?;
        ui_ctx.update(base_ui.update(&log_view));
    }
}
