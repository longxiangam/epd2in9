use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use embedded_graphics_core::geometry::Point;
use crate::widgets::Widget;

#[derive( Clone)]
pub struct Wrap{
    position:Point,
    width:i32,
    height:i32,
    children: Vec<Box<dyn Widget>>
}

impl Wrap{
    pub fn new(position: Point, width:i32, height:i32) -> Wrap {
        Self{
            position,
            width,
            height,
            children: vec![],
        }
    }
    pub fn add_child(&mut self, child :Box<dyn Widget>){
        self.children.push(child);
    }
}
impl Clone for Wrap {
    fn clone(&self) -> Self {
        Wrap {
            position: self.position,
            width: self.width,
            height: self.height,
            children: self.children.clone(),
        }
    }
}




impl Widget for Wrap{
    fn draw(&self) {
        todo!()
    }
}