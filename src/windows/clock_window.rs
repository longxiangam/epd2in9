use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use embedded_graphics_core::geometry::Point;
use crate::app::MainApp;
use crate::widgets::label::Label;
use crate::widgets::wrap::Wrap;
use crate::windows::Window;

pub struct ClockWindow<'a>{
    pub root: Wrap,
    pub app:Rc<RefCell<MainApp<'a>>> ,
}

impl <'a> ClockWindow<'a> where 'a:'static{
    pub fn new(app: Rc<RefCell<MainApp<'a>>>, width:i32, height:i32) -> ClockWindow<'a>

    {

        let mut root = Wrap::new(Point::new(0,0),width,height);
        root.add_child(Box::new(Label::new(Point::new(10,10),100,"时钟界面")));

        Self{
            root,
            app,
        }
    }

    fn back(&self){
        self.app.borrow_mut().pop();
    }

}




impl <'a> Window<'a> for ClockWindow<'a>{
    fn run(&self) {
        todo!()
    }

    fn draw(&self) {

    }

    /*    fn listen_event(&self, callback: impl FnOnce()) {
            callback();
        }*/
}
