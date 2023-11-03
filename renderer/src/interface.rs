use crate::Body;
use screen::screen::{Handler, SCREEN_SIZE_X, SCREEN_SIZE_Y};
use utils::Pos;

const OMEGA: f32 = 0.6;

pub struct ScreenHandler {
    screen: [[char; SCREEN_SIZE_X]; SCREEN_SIZE_Y],
    z_buffer: [[f32; SCREEN_SIZE_X]; SCREEN_SIZE_Y],
    center_pos: Pos,
}

impl ScreenHandler {
    pub fn new() -> Self {
        ScreenHandler {
            screen: [[' '; SCREEN_SIZE_X]; SCREEN_SIZE_Y],
            z_buffer: [[10000.0; SCREEN_SIZE_X]; SCREEN_SIZE_Y],
            center_pos: Pos {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
        }
    }
}

impl ScreenHandler {
    pub fn is_drawable_z(&mut self, x_index: usize, y_index: usize, z: f32) -> bool {
        return z < self.z_buffer[y_index][x_index];
    }

    pub fn set_screen(&mut self, x_index: usize, y_index: usize, luminance: char) {
        self.screen[y_index][x_index] = luminance;
    }

    pub fn set_z_buffer(&mut self, x_index: usize, y_index: usize, z: f32) {
        self.z_buffer[y_index][x_index] = z;
    }

    pub fn get_z_buffer(&mut self, x_index: usize, y_index: usize) -> f32 {
        self.z_buffer[y_index][x_index]
    }
}

impl Handler for ScreenHandler {
    fn clear(&mut self) {
        for xi in 0..SCREEN_SIZE_X {
            for yi in 0..SCREEN_SIZE_Y {
                self.screen[yi][xi] = ' ';
                self.z_buffer[yi][xi] = 10000.0;
            }
        }
    }

    fn get_screen(
        &mut self,
    ) -> [[char; screen::screen::SCREEN_SIZE_X]; screen::screen::SCREEN_SIZE_Y] {
        self.screen.clone()
    }

    fn tick(
        &mut self,
        objs: &mut Vec<Box<dyn screen::screen::Drawable>>,
        time: &screen::screen::Time,
    ) {
        for obj in objs
            .iter_mut()
            .map(|x| x.as_any_mut().downcast_ref::<Body>().unwrap())
        {
            // Rotate position
            let rot_pos = obj.rotate_around(time, OMEGA, &self.center_pos);

            // Draw on screen
            self.set_point_illuminated_world(rot_pos, obj.is_water);
        }
    }
}
