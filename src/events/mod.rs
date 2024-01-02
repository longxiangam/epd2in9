
use alloc::rc::Rc;

use core::cell::RefCell;

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




pub struct EventListener<'a>{
    pub app:Rc<RefCell<MainApp<'a>>> ,
}


impl  EventListener<'_>{

    fn trigger(&self,event_type: EventType){
        self.app.borrow_mut().top_window().process_event(event_type);

    }
}
