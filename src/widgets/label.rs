
use crate::widgets::{Point, Widget};
#[derive(Copy, Clone)]
pub struct Label<'a> {
    position:Point,
    width:i32,
    text:& 'a str,
}

impl <'a> Label<'a>{
    pub fn new(position:Point, width:i32, text:&'a str) -> Label<'a> {
        Self{
            position,
            width,
            text
        }
    }
}

impl <'a> Widget for Label<'a>{
    fn draw(&self) {
        todo!()
    }
}