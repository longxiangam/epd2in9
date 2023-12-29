use crate::events::EventType;

pub(crate) mod menu_window;
mod clock_window;


pub(crate) trait Window<'a> {
    fn run(&self);

    fn draw(&self);

}
