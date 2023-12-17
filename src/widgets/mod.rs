use embedded_graphics_core::prelude::Point;

pub struct Label<'a> {
    position:Point,
    width:i32,
    text:& 'a str,
}

impl Label{
    pub fn new<'a>(position:Point, width:i32, text:&str) -> Label<'a> {
        Self{
            position,
            width,
            text
        }
    }
}

pub struct  Button<'a>{
    position:Point,
    width:i32,
    text:& 'a str,
}


pub trait  Widget{

    fn render();
}