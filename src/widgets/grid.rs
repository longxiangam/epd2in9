use alloc::boxed::Box;
use crate::widgets::{Point, Widget};
#[derive(Copy, Clone)]
pub struct Grid<'a,const R:usize,const C:usize>{
    position:Point,
    width:i32,
    height:i32,
    cells: [GridCell<'a>;R]
}

#[derive(Copy, Clone)]
pub struct  GridCell<'a>{
    active:bool,
    width:i32,
    height:i32,
    text:& 'a str,
}