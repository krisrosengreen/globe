use std::ops::{Add, Sub};

use screen::screen::{SCREEN_SIZE_X, SCREEN_SIZE_Y, Y_SQUISH};

pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct ScreenPos {
    pub x: f32,
    pub y: f32 
}

impl ScreenPos {
    pub fn is_drawable(x: f32, y: f32) -> bool {
        x > 0.0 && y > 0.0 && x < (SCREEN_SIZE_X - 1) as f32 && y * Y_SQUISH < (SCREEN_SIZE_Y - 1) as f32
    }

    pub fn to_indeces(&self) -> [usize; 2] {
        [self.x.floor() as usize, (self.y * Y_SQUISH).floor() as usize]
    }
}

impl Pos {
    pub fn rotate(&self, angle: f32) -> Pos {
        // cos(theta) 0  sin(theta)
        // 0          1  0
        // sin(theta) 0 -cos(theta)
        let x = angle.cos()*self.x + angle.sin()*self.z;
        let y = self.y;
        let z = angle.sin()*self.x - angle.cos() * self.z;

        Pos {
            x,
            y,
            z
        }
    }
}

impl Add<&Pos> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub<&Pos> for &Pos {
    type Output = Pos;
    fn sub(self, rhs: &Pos) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}