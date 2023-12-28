use alloc::boxed::Box;
use alloc::rc::Rc;
use embedded_graphics_core::prelude::Point;
use crate::app::MainApp;
use crate::widgets::label::Label;
use crate::widgets::wrap::Wrap;

pub struct MenuWindow{
    pub root: Wrap,
    pub app:Rc<MainApp>,
}

impl MenuWindow{
    pub fn new(app: Rc<MainApp>, width:i32, height:i32) -> Self{


        let mut root = Wrap::new(Point::new(0,0),width,height);
        root.add_child(Box::new(Label::new(Point::new(10,10),100,"测试label")));

        Self{
            root,
            app,
        }
    }

}

impl Clone for MenuWindow {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Copy for MenuWindow {}

impl Window for MenuWindow{
    fn run(&self) {
        todo!()
    }

    fn draw(&self) {

    }

/*    fn listen_event(&self, callback: impl FnOnce()) {
        callback();
    }*/
}

pub(crate) trait Window : Copy{
    fn run(&self);

    fn draw(&self);

 /*   fn listen_event(&self, callback: impl FnOnce());*/
}
