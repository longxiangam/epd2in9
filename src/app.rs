use alloc::boxed::Box;

use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::text::Text;
use embedded_graphics_core::Drawable;
use embedded_graphics_core::geometry::Point;
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_graphics_core::prelude::DrawTarget;


use crate::windows::{ Window};
extern crate alloc;
pub const SCREEN_WIDTH:i32 = 400;
pub const SCREEN_HEIGHT:i32 = 300;


pub struct MainApp<'a,D> where D: DrawTarget<Color = BinaryColor>{
    pub  window_stack:Vec<Box<dyn  Window<'a, D>> >,
    pub need_render:bool,
    pub is_pause:bool,
}

impl <'a,D> MainApp<'a,D>  where D: DrawTarget<Color = BinaryColor> {
    pub fn new() -> MainApp<'a,D> {
        let mut app = Self{
            window_stack:vec![],
            need_render:true,
            is_pause:false,
        };
        app

    }


    pub  fn top_window(&mut self) -> &mut Box<(dyn Window<'a, D>)> {
        if let Some(last_window) = self.window_stack.last_mut() {
            return last_window;
        } else {
            // 处理空 vec 的情况，这里可以根据实际需求选择 panic 或者返回默认值
            panic!("Window stack is empty!");
        }
    }

    pub  fn push(&mut self, window:Box<(dyn Window<'a, D>)>) {
        self.window_stack.push(window);
        //重绘
        self.need_render = true;
    }

    pub  fn pop(&mut self){
        self.window_stack.pop();
        //重绘
        self.need_render = true;
    }

    pub  fn pause(&mut self){
        self.is_pause = true;
    }

    pub fn resume(&mut self){
        self.is_pause = false;
    }

    pub fn run(&mut self, display:&mut D){
        if !self.is_pause {
            if self.need_render {
                let window = self.top_window();
                window.draw(display);

            }

            //self.top_window().run();
        }
    }
}

