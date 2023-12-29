pub(crate) mod menu_window;
mod clock_window;


pub(crate) trait Window<'a> {
    fn run(&self);

    fn draw(&self);

 /*   fn listen_event(&self, callback: impl FnOnce());*/
}
