use alloc::boxed::Box;
use embedded_graphics_core::geometry::Point;
use crate::widgets::Widget;

pub struct Wrap{
    position:Point,
    width:i32,
    height:i32,
    child:[Box<dyn Widget>]
}