use crate::widgets::Point;

#[derive(Copy, Clone)]
pub struct  Button<'a>{
    position:Point,
    width:i32,
    text:& 'a str,
}

impl <'a> Button<'a>{
    pub fn new(position:Point, width:i32, text:&'a str) -> Button<'a> {
        Self{
            position,
            width,
            text
        }
    }
}