use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use embedded_graphics_core::prelude::Point;
use crate::app::MainApp;

use crate::widgets::wrap::Wrap;

pub struct MenuWindow<'a>{
    pub root: Wrap,
    pub app:Rc<RefCell<MainApp<'a>>> ,
}

impl <'a> MenuWindow<'a>{
    pub fn new(app: Rc<RefCell<MainApp<'a>>>, width:i32, height:i32) -> MenuWindow<'a>{


        let mut root = Wrap::new(Point::new(0,0),width,height);
        //root.add_child(Box::new(Label::new(Point::new(10,10),100,"测试label")));

        Self{
            root,
            app,
        }
    }

}




impl <'a> Window<'a> for MenuWindow<'a>{
    fn run(&self) {
        todo!()
    }

    fn draw(&self) {

    }

/*    fn listen_event(&self, callback: impl FnOnce()) {
        callback();
    }*/
}

pub(crate) trait Window<'a> {
    fn run(&self);

    fn draw(&self);

 /*   fn listen_event(&self, callback: impl FnOnce());*/
}
