use std::ops::{Add, Div, Mul, Sub};

use screen::screen::{SCREEN_SIZE_X, SCREEN_SIZE_Y, Y_SQUISH};

#[derive(Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct ScreenPos {
    pub x: f32,
    pub y: f32,
}

impl ScreenPos {
    pub fn is_drawable(x: f32, y: f32) -> bool {
        let sx = x + SCREEN_SIZE_X as f32 / 2.0;
        let sy = y + SCREEN_SIZE_Y as f32 / 2.0;

        sx > 0.0
            && sy > 0.0
            && sx < SCREEN_SIZE_X as f32 - 1.0
            && sy * Y_SQUISH < SCREEN_SIZE_Y as f32 - 1.0
    }

    pub fn to_indeces(&self) -> [usize; 2] {
        [
            (self.x + SCREEN_SIZE_X as f32 / 2.0).floor() as usize,
            ((self.y + SCREEN_SIZE_Y as f32 / 2.0) * Y_SQUISH).floor() as usize,
        ]
    }
}

impl Pos {
    // Rotates on the x and z plane
    pub fn rotate(&self, angle: f32) -> Pos {
        // cos(theta) 0  sin(theta)
        // 0          1  0
        // sin(theta) 0 -cos(theta)
        let x = angle.cos() * self.x + angle.sin() * self.z;
        let y = self.y;
        let z = angle.sin() * self.x - angle.cos() * self.z;

        Pos { x, y, z }
    }

    pub fn size(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&mut self) {
        let size = self.size();
        
        self.x /= size;
        self.y /= size;
        self.z /= size;
    }
}

impl Add<&Pos> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<&Pos> for &Pos {
    type Output = Pos;
    fn sub(self, rhs: &Pos) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f32> for &Pos {
    type Output = Pos;
    fn div(self, rhs: f32) -> Self::Output {
        Pos {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f32> for &Pos {
    type Output = Pos;
    fn mul(self, rhs: f32) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_add() {
        let pos1 = Pos {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let pos2 = Pos {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };

        let added = &pos1 + &pos2;

        assert!(added.x == 3.0);
    }

    #[test]
    fn test_pos_sub() {
        let pos1 = Pos {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let pos2 = Pos {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        };

        let added = &pos1 - &pos2;

        assert!(added.x == -1.0);
    }

    #[test]
    fn test_size() {
        let pos1 = Pos {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };

        assert!(pos1.size() == 4.0);
    }

    #[test]
    fn test_rotate() {
        let pos1 = Pos {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };

        let rot_pos = pos1.rotate(3.1415);

        println!(
            "pos.rotate {} {} {} - {}",
            rot_pos.x,
            rot_pos.y,
            rot_pos.z,
            rot_pos.size()
        );
        assert!(rot_pos.x == -4.0);
    }
}
