use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Deref;
use embedded_graphics_core::prelude::Point;

use crate::widgets::Widget;
use crate::widgets::wrap::Wrap;
use crate::windows::{MenuWindow, Window};

pub const SCREEN_WIDTH:i32 = 400;
pub const SCREEN_HEIGHT:i32 = 300;


pub struct MainApp<'a>{
    window_stack:Vec<Box<dyn  Window<'a>> >,
    need_render:bool,
    is_pause:bool,
}

impl <'a> MainApp<'a>{
    pub fn new() -> MainApp<'a> {
       let mut app = Self{
            window_stack:vec![],
            need_render:true,
            is_pause:false,
        };
        app

    }


    pub  fn top_window(&mut self) -> &mut Box<dyn Window<'a>> {
        if let Some(last_window) = self.window_stack.last_mut() {
            return last_window;
        } else {
            // 处理空 vec 的情况，这里可以根据实际需求选择 panic 或者返回默认值
            panic!("Window stack is empty!");
        }
    }

    pub  fn push(&mut self, window: Box<dyn Window<'a> >){
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

    pub   fn run(&mut self)->!{
        while self.is_pause {
            if self.need_render {
                self.top_window().draw();
            }

            self.top_window().run();
        }
        loop{}
    }
}

