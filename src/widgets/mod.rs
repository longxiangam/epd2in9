use embedded_graphics_core::geometry::Point;
pub mod label;
pub mod button;
pub mod grid;
pub mod list;
pub mod wrap;



pub trait  Widget :Copy{
    fn draw(&self);
}