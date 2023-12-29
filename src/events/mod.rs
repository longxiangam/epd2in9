use alloc::boxed::Box;
use alloc::vec::Vec;
use core::iter::Map;
use crate::windows::Window;

pub enum  EventType{
    ButtonAShort,
    ButtonADouble,
    ButtonALong,
    ButtonBShort,
    ButtonBDouble,
    ButtonBLong,
}


pub struct EventQueue<'a>{
    queue:Vec<EventType>,
    listener:Map<EventType,&'a dyn Window<'a>>
}
impl  EventQueue<'_>{
    fn trigger(){

    }
}