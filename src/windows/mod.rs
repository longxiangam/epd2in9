use alloc::boxed::Box;
use crate::widgets::{Widget};

pub struct Window{
    pub widgets:[Box<dyn Widget>]
}
