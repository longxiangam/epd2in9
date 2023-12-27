use alloc::boxed::Box;
use crate::widgets::{Point, Widget};

pub struct  Grid<'a>{
    position:Point,
    width:i32,
    height:i32,
    row:i32,
    col:i32,
    cells:[GridCell<'a>]
}


pub struct  GridCell<'a>{
    active:bool,
    width:i32,
    height:i32,
    text:& 'a str,
}