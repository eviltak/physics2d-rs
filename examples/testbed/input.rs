use physics2d::*;
use sfml;

use testbed::physics2d_vec2;

pub struct Input {
    mouse_position: Vec2,
}

impl Input {
    pub(super) fn collect(window: &sfml::graphics::RenderWindow, pixels_per_unit: f32) -> Input {
        let mouse_pos = window.mouse_position();
        Input {
            mouse_position: physics2d_vec2(
                sfml::system::Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32),
                pixels_per_unit
            ),
        }
    }
    
    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
}
