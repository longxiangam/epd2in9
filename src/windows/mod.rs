use crate::events::EventType;

pub mod menu_window;
pub mod clock_window;


pub trait Window<'a> {
    fn run(&self);

    fn draw(&self);

    fn process_event(&self,event_type: EventType){}

}
