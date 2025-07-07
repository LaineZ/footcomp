use edgy::{
    themes::{self},
    UiContext,
};
use edgy::embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*
};
use embedded_graphics_simulator::{sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let output_settings = OutputSettingsBuilder::new()
        .pixel_spacing(0)
        .scale(4)
        .build();

    let mut window = Window::new("a bit edgy ui", &output_settings);
    let mut ui_ctx = UiContext::new(display, themes::hope_diamond::apply());

    loop {
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
        ui_ctx.update(footcomp_ui::base_ui());
    }
}
