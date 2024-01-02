use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use embedded_graphics_core::geometry::Point;
use crate::app::MainApp;
use crate::events::EventType;
use crate::widgets::label::Label;
use crate::widgets::wrap::Wrap;
use crate::windows::clock_window::ClockWindow;
use crate::windows::Window;
extern crate alloc;
pub struct MenuWindow<'a>{
    pub root: Wrap,
    pub need_render:bool,
    pub app:Rc<RefCell<MainApp<'a>>> ,
}

impl <'a> MenuWindow<'a> where 'a:'static{
    pub fn new(app: Rc<RefCell<MainApp<'a>>>, width:i32, height:i32) -> MenuWindow<'a>

    {

        let mut root = Wrap::new(Point::new(0,0),width,height);
        root.add_child(Box::new(Label::new(Point::new(10,10),100,"测试label")));

        let window = Self{
            root,
            need_render:false,
            app,
        };

        window
    }

    fn to_clock(&self){
        let window:Box<dyn Window<'a>> = Box::new(ClockWindow::new(self.app.clone(), crate::app::SCREEN_WIDTH, crate::app::SCREEN_HEIGHT));
        self.app.borrow_mut().push(window);
    }

}




impl <'a> Window<'a> for MenuWindow<'a>{
    fn run(&self) {

    }

    fn draw(&self) {

    }

}
