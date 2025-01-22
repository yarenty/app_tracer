use termion::event;


#[derive(Debug)]
pub enum Event {
    Input(event::Key),
    Tick,
    Quit,
}
