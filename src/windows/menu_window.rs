use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::fmt::Debug;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::text::Text;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::Drawable;
use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::pixelcolor::BinaryColor;
use crate::app::MainApp;
use crate::events::EventType;
use crate::widgets::label::Label;
use crate::widgets::wrap::Wrap;
use crate::windows::clock_window::ClockWindow;
use crate::windows::Window;
extern crate alloc;
pub struct MenuWindow<'a,D> where D: DrawTarget<Color = BinaryColor> {
    pub root: Wrap,
    pub need_render:bool,
    pub app:Rc<RefCell<MainApp<'a,D>>> ,
}

impl <'a,D> MenuWindow<'a,D> where 'a:'static,D: DrawTarget<Color = BinaryColor> + 'a {
    pub fn new(app: Rc<RefCell<MainApp<'a, D>>>, width:i32, height:i32) -> MenuWindow<'a,D>

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




impl <'a,D> Window<'a> for MenuWindow<'a,D>  where D: DrawTarget<Color =BinaryColor> {
    fn run(&self) {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        Text::new(
            "This is a\nmultiline\nHello World!",
            Point::new(15, 15),
            style,
        )
            .draw(&mut self.app.clone().borrow_mut().display);//.expect("绘制失败");
    }

    fn draw(&self) {

    }

}
