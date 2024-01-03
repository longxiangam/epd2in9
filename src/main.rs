#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use core::cell::RefCell;

use esp_backtrace as _;
use esp_println::println;
use esp_println::print;


use hal::riscv::_export::critical_section;
use hal::riscv::_export::critical_section::Mutex;
use hal::prelude::*;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, IO
          , gpio::{Gpio0,Gpio1, Input, PullDown}, Delay, esp_riscv_rt, interrupt, peripherals, riscv, gpio};

use epd_waveshare::{epd2in9::*, graphics::DisplayRotation, prelude::*};
use epd_waveshare::epd2in9::{Display2in9, Epd2in9, HEIGHT, WIDTH};
use epd_waveshare::prelude::{Display, WaveshareDisplay};
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_graphics::{
    pixelcolor::BinaryColor::On as Black,
    pixelcolor::BinaryColor::{Off as White},

};

use hal::system::Peripheral;
use crate::app::MainApp;
use crate::events::EventListener;
use crate::windows::menu_window::MenuWindow;


pub mod widgets;
mod windows;
mod events;
mod app;
mod driver;


static BUTTON: Mutex<RefCell<Option<Gpio0<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));
static BUTTON1: Mutex<RefCell<Option<Gpio1<Input<PullDown>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {

    // -------- Setup Allocator --------
    const HEAP_SIZE: usize = 100 * 1024;
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    #[global_allocator]
    static ALLOCATOR: embedded_alloc::Heap = embedded_alloc::Heap::empty();
    unsafe { ALLOCATOR.init(&mut HEAP as *const u8 as usize, core::mem::size_of_val(&HEAP)) };


    println!("Hello, world!");

    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO,peripherals.IO_MUX);


    //墨水屏
    let epd_sclk = io.pins.gpio2;
    let epd_mosi = io.pins.gpio3;
    let epd_cs = io.pins.gpio7.into_push_pull_output();
    let epd_rst =io.pins.gpio10.into_push_pull_output();
    let epd_dc = io.pins.gpio6.into_push_pull_output();

    let mut spi = hal::spi::master::Spi::new(
        peripherals.SPI2,

        32u32.MHz(),
        hal::spi::SpiMode::Mode0,
        &clocks,
    );
    spi = spi.with_sck(epd_sclk);
    spi = spi.with_mosi(epd_mosi);

    let busy_in = io.pins.gpio11.into_pull_up_input();

    let mut epd = Epd2in9::new(&mut spi, epd_cs, busy_in, epd_dc, epd_rst, &mut delay).unwrap();

    let mut display: Display2in9 = Display2in9::default();
    let mut main_app =  crate::MainApp::new();
    let rc = Rc::new(RefCell::new(main_app));

    let window = Box::new(MenuWindow::new(rc.clone(), epd2in9::app::SCREEN_WIDTH, epd2in9::app::SCREEN_HEIGHT));


    rc.clone().borrow_mut().push(window);


    //let event_listener:EventListener = EventListener{app:rc.clone()};


    rc.clone().borrow_mut().run(&mut display);

    //let event_listener:EventListener = EventListener{app:rc.clone()};

    loop {

        delay.delay_ms(500u32);
    }
}