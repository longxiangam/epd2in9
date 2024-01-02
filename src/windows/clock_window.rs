use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::pixelcolor::BinaryColor;
use crate::app::MainApp;
use crate::widgets::label::Label;
use crate::widgets::wrap::Wrap;
use crate::windows::Window;
extern crate alloc;
pub struct ClockWindow<'a,D> where D: DrawTarget<Color = BinaryColor>{
    pub root: Wrap,
    pub app:Rc<RefCell<MainApp<'a,D>>> ,
}

impl <'a,D> ClockWindow<'a,D> where 'a:'static,D: DrawTarget<Color =BinaryColor> {
    pub fn new(app: Rc<RefCell<MainApp<'a,D>>>, width:i32, height:i32) -> ClockWindow<'a,D>
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




impl <'a,D> Window<'a, D> for ClockWindow<'a,D>  where D: DrawTarget<Color = BinaryColor> {
    fn run(&self,display: &mut  D) {
        todo!()
    }

    fn draw(&self,display:&mut D) {

    }


}
