
use crate::widgets::{Point, Widget};

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