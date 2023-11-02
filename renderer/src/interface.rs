use screen::screen::{Handler, Time, SCREEN_SIZE_X, SCREEN_SIZE_Y};

const SCREEN_Y_OFFSET: f32 = 2.0;

use crate::{
    graphics::{LUMINANCE, LUMINANCE_COUNT, ScreenPosUtils},
    Body,
};

use utils::{
    utils::Pos,
    utils::ScreenPos,
};

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

    fn set_point(&mut self, pos: Pos, luminance: char) {
        if !ScreenPos::is_drawable(pos.x, pos.y + SCREEN_Y_OFFSET) {
            println!("Not drawable!");
            return;
        }

        let screen_pos_offset = Pos {
            x: pos.x,
            y: pos.y + SCREEN_Y_OFFSET,
            z: pos.z
        };

        let screen_pos = screen_pos_offset.to_screen_pos();
        let indeces = screen_pos.to_indeces();

        if self.z_buffer[indeces[1]][indeces[0]] > pos.z {
            self.screen[indeces[1]][indeces[0]] = luminance;
            self.z_buffer[indeces[1]][indeces[0]] = pos.z;
        }
    }

    fn set_point_illuminated(&mut self, pos: Pos, time: &Time) {
        let start_light_direction = Pos {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let light_omega: f32 = 0.2;

        let current_ligh_direction = start_light_direction.rotate(light_omega * time.current_time);

        let center_pos = Pos {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let diff = &pos - &center_pos;
        let diff = &diff / diff.size();

        let dot = diff.x * current_ligh_direction.x
            + diff.y * current_ligh_direction.y
            + diff.z * current_ligh_direction.z;

        if dot < 0.0 {
            self.set_point(pos, '.');
        } else {
            let char_index = (dot * (LUMINANCE_COUNT as f32) - 1.0).floor() as usize;

            self.set_point(pos, LUMINANCE[char_index]);
        }
    }

    fn set_point_illuminated_world(&mut self, pos: Pos, time: &Time, is_water: bool) {
        let mut start_light_direction = Pos {
            x: -0.5,
            y: 0.0,
            z: -0.5,
        };

        start_light_direction.normalize();

        let light_omega: f32 = 1.0;

        let current_ligh_direction = start_light_direction;
        //let current_ligh_direction = start_light_direction.rotate(light_omega * time.current_time);

        let center_pos = Pos {
            x: 0.0,
            y: 0.0,
            z: 10.0,
        };

        let diff = &pos - &center_pos;
        let diff = &diff / diff.size();

        let dot = diff.x * current_ligh_direction.x
            + diff.y * current_ligh_direction.y
            + diff.z * current_ligh_direction.z;

        /*
        if is_water {
            self.set_point(pos, '#');
        } else {
            self.set_point(pos, '.');
        }
        */

        if dot < 0.0 || is_water {
            self.set_point(pos, '.');
        } else {
            let char_index = (dot * (LUMINANCE_COUNT as f32) - 1.0).floor() as usize;

            self.set_point(pos, LUMINANCE[char_index]);
            //self.set_point(pos, '#');
        }
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
            let rot_pos = obj.rotate_around(time, OMEGA, &self.center_pos);

            //println!("pos: {:?}", rot_pos);

            // self.set_point_illuminated(rot_pos, time);

            self.set_point_illuminated_world(rot_pos, time, obj.is_water);

            //self.set_point(rot_pos, '#');
        }
    }
}
