use core::fmt::Debug;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;
use crate::events::EventType;

pub mod menu_window;
pub mod clock_window;


pub  trait Window<'a> {
    fn run(&self) ;

    fn draw(&self);

    fn process_event(&self,event_type: EventType){}

}
