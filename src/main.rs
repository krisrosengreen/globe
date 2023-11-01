mod interface;
mod utils;
mod graphics;

use screen::screen::*;
use utils::Pos;

pub struct Body {
    pos: Pos
}

impl Body {
    pub fn rotate_around(&self, time: &Time, omega: f32, center_pos: &Pos) -> Pos {
        let diff = &self.pos - center_pos;
        let rotated_pos = diff.rotate(omega*time.current_time);

        &rotated_pos+&self.pos
    }
}

impl Drawable for Body {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn setup_points(screen: &mut Screen,) {
    let mid_x = (SCREEN_SIZE_X / 2) as f32;
    let mid_y = (SCREEN_SIZE_Y / 2) as f32;

    let body_one = Body {
        pos: Pos { x: 5.0+mid_x, y: 5.0+mid_y, z: 7.0 }
    };

    let body_two = Body {
        pos: Pos { x: -5.0+mid_x, y: 5.0+mid_y, z: 7.0 }
    };

    let body_three = Body {
        pos: Pos { x: 0.0+mid_x, y: 5.0+mid_y, z: 15.0 }
    };

    screen.add_body(Box::new(body_one));
    screen.add_body(Box::new(body_two));
    screen.add_body(Box::new(body_three))
}

fn setup_screen() {
    let time = Time {current_time: 0.0};

    let handler = interface::ScreenHandler::new();

    let mut screen = Screen::new(Box::new(handler));

    setup_points(&mut screen);

    screen_loop(screen, time)
}

fn main() {
    setup_screen();
}
