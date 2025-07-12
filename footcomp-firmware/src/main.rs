#![no_std]
#![no_main]
use edgy::themes::hope_diamond;
use edgy::UiContext;
use footcomp_ui::views::log::LogPage;
use footcomp_ui::views::main::MainPage;
use panic_probe as _;
use rp2040_hal::fugit::RateExtU32;
use rp2040_hal::gpio::{FunctionI2C, Pin};
use rp2040_hal::{clocks::init_clocks_and_plls, gpio::Pins, pac, Clock, Sio, Watchdog};
use rp2040_hal::{entry, I2C};
use sh1106::mode::GraphicsMode;

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 8192;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        #[allow(static_mut_refs)]
        unsafe {
            HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE)
        }
    }

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio18.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio19.reconfigure();

    let i2c = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        500.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let mut disp: GraphicsMode<_> = sh1106::Builder::new().connect_i2c(i2c).into();
    let mut ui_context = UiContext::new(disp, hope_diamond::apply());

    ui_context.draw_target.init();
    ui_context.draw_target.flush();

    let base_ui = footcomp_ui::BaseUi::default();
    let main_view = MainPage::default();
    let log_view = LogPage::default();

    loop {
        ui_context.draw_target.clear();
        ui_context.update(base_ui.update(&log_view));
        let _ = ui_context.draw_target.flush();
    }
}

// End of file
