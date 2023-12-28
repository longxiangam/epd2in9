use alloc::vec::Vec;
use crate::widgets::grid::GridCell;
use crate::widgets::Point;
#[derive( Clone)]
pub struct  List<'a>{
    position:Point,
    width:i32,
    height:i32,
    col:i32,
    rows: Vec<ListRow<'a>>
}
#[derive(Copy, Clone)]
pub struct  ListRow<'a>{
    active:bool,
    height:i32,
    text:& 'a str,
}