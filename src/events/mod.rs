
use alloc::rc::Rc;

use core::cell::RefCell;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;

use crate::app::MainApp;
use crate::windows::Window;
extern crate alloc;
pub enum  EventType{
    ButtonAShort,
    ButtonADouble,
    ButtonALong,
    ButtonBShort,
    ButtonBDouble,
    ButtonBLong,
}




pub struct EventListener<'a,D>  where D: DrawTarget<Color = BinaryColor> {
    pub app:Rc<RefCell<MainApp<'a,D>>> ,
}


impl <'a,D> EventListener<'a,D>  where D: DrawTarget<Color = BinaryColor> {

    fn trigger(&self,event_type: EventType){
        self.app.borrow_mut().top_window().process_event(event_type);

    }
}
