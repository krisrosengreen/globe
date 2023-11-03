mod graphics;
mod interface;
mod setup;

use graphics::Body;
use screen::screen::{Time, Screen, screen_loop};

fn setup_system() {
    let time = Time { current_time: 0.0 };

    let handler = interface::ScreenHandler::new();

    let mut screen = Screen::new(Box::new(handler));

    setup::setup_world_points(&mut screen);

    screen_loop(screen, time)
}

fn main() {
    setup_system();
}
