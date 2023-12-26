
use crate::widgets::Point;

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