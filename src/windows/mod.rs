use alloc::vec::Vec;
use crate::widgets::{Button, Widget};

pub struct Window{
    pub widgets:Vec<dyn Widget>
}
