use screen::screen::{Drawable, Time};

use utils::utils::{Pos, ScreenPos};

pub const SCREEN_DIST_Z: f32 = 50.0;

#[allow(unused)]
pub const LIGHT_DIRECTION: Pos = Pos {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const LUMINANCE: [char; 69] = [
    '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?',
    ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c',
    'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h',
    'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
];
pub const LUMINANCE_COUNT: u8 = 69;

pub struct Body {
    pub pos: Pos,
    pub is_water: bool
}

impl Body {
    // Rotate around center_pos on the xz-plane
    pub fn rotate_around(&self, time: &Time, omega: f32, center_pos: &Pos) -> Pos {
        let diff = &self.pos - center_pos;

        let rotated_pos = diff.rotate(omega * time.current_time);

        let ret = &rotated_pos + &center_pos;

        ret
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

pub trait ScreenPosUtils {
    fn to_screen_pos(&self) -> ScreenPos;
}

impl ScreenPosUtils for Pos {
    fn to_screen_pos(&self) -> ScreenPos {
        let scale_factor = SCREEN_DIST_Z / self.z;

        ScreenPos {
            x: self.x * scale_factor,
            y: self.y * scale_factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOAT_EPSILON: f32 = 0.00001;

    #[test]
    fn test_body_rotation() {
        let center_pos = Pos {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let test_pos = Pos {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        };

        let test_body = Body { pos: test_pos, is_water: false};

        let time = Time { current_time: 1.0 };

        let rotated_pos = test_body.rotate_around(&time, 3.1415, &center_pos);

        println!(
            "rotpos {} {} {} - {}",
            rotated_pos.x,
            rotated_pos.y,
            rotated_pos.z,
            rotated_pos.size()
        );

        assert!((rotated_pos.size() - 10.0).abs() < FLOAT_EPSILON);
    }
}
