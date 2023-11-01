use screen::screen::{Handler, SCREEN_SIZE_X, SCREEN_SIZE_Y};

use crate::{Body, utils::Pos, utils::ScreenPos};

const OMEGA: f32 = 0.6;

pub struct ScreenHandler {
    screen: [[char; SCREEN_SIZE_X]; SCREEN_SIZE_Y],
    center_pos: Pos
}

impl ScreenHandler {
    pub fn new() -> Self {
        ScreenHandler { screen: [[' '; SCREEN_SIZE_X]; SCREEN_SIZE_Y],
        center_pos: Pos {
            x: (SCREEN_SIZE_X / 2) as f32,
            y: (SCREEN_SIZE_Y / 2) as f32,
            z: 10.0
            }
        }
    }

    fn set_point(&mut self, pos: Pos, luminance: char) {
        if !ScreenPos::is_drawable(pos.x, pos.y) {
            return;
        }

        let indeces = pos.to_screen_pos().to_indeces();

        self.screen[indeces[1]][indeces[0]] = luminance;
    }
}

impl Handler for ScreenHandler {
    fn clear(&mut self) {
        for xi in 0..SCREEN_SIZE_X {
            for yi in 0..SCREEN_SIZE_Y {
                self.screen[yi][xi] = ' ';
            }
        }
    }

    fn get_screen(&mut self) -> [[char; screen::screen::SCREEN_SIZE_X]; screen::screen::SCREEN_SIZE_Y] {
        self.screen.clone()
    }

    fn tick(&mut self, objs: &mut Vec<Box<dyn screen::screen::Drawable>>, time: &screen::screen::Time) {
        for obj in objs.iter_mut().map(|x| x.as_any_mut().downcast_ref::<Body>().unwrap()) {
            let rot_pos = obj.rotate_around(time, OMEGA, &self.center_pos);

            let rot_pos_diff = &rot_pos - &self.center_pos;

            println!("pos: {} {} {}", rot_pos_diff.x, rot_pos_diff.y, rot_pos_diff.z);

            self.set_point(rot_pos, '#');
        }
    }
}