use std::cell::RefCell;
use std::rc::Rc;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use epd2in9::app::MainApp;
use epd2in9::events::EventListener;
use epd2in9::windows::menu_window::MenuWindow;


fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(256, 128));

    let style = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();

    Text::new(
        "This is a\nmultiline\nHello World!",
        Point::new(15, 15),
        style,
    )
        .draw(&mut display)?;


    let mut main_app: MainApp =  crate::MainApp::new();
    let rc = Rc::new(RefCell::new(main_app));

    let window = Box::new(MenuWindow::new(rc.clone(), epd2in9::app::SCREEN_WIDTH, epd2in9::app::SCREEN_HEIGHT));

    rc.clone().borrow_mut().push(window);

    let event_listener:EventListener = EventListener{app:rc.clone()};


      //  rc.clone().borrow_mut().run();



    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}