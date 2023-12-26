use crate::widgets::grid::GridCell;
use crate::widgets::Point;

pub struct  List<'a>{
    position:Point,
    width:i32,
    height:i32,
    col:i32,
    rows:[ListRow<'a>]
}

pub struct  ListRow<'a>{
    height:i32,
    text:& 'a str,
}