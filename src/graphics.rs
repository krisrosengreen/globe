use crate::utils::{Pos, ScreenPos};

pub const SCREEN_DIST_Z: f32 = 1.0; 

impl Pos {
    pub fn to_screen_pos(&self) -> ScreenPos {
        let scale_factor = SCREEN_DIST_Z / self.z;

        ScreenPos { x: self.x*scale_factor, y: self.y*scale_factor }
    }
}