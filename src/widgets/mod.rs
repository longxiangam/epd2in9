use embedded_graphics_core::geometry::Point;
mod label;
mod button;
mod grid;
mod list;
mod wrap;


pub trait  Widget{
    fn draw(&self);
}