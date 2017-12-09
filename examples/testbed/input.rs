use physics2d::*;
use sfml;
use sfml::graphics::RenderTarget;

use testbed::physics2d_vec2;

pub struct Input {
    pub mouse_position: Vec2,
    
    pub left_mouse_pressed: bool,
    pub left_mouse_released: bool,
    
    pub right_mouse_pressed: bool,
    pub right_mouse_released: bool,
}

impl Input {
    pub(super) fn new() -> Input {
        Input {
            mouse_position: Vec2::ZERO,
            left_mouse_pressed: false,
            left_mouse_released: false,
            right_mouse_pressed: false,
            right_mouse_released: false,
        }
    }
    
    pub(super) fn collect(&mut self, window: &sfml::graphics::RenderWindow, pixels_per_unit: f32) {
        let mouse_pos = window.mouse_position();
        let mut mouse_pos = sfml::system::Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
        mouse_pos -= window.view().center() + window.view().size() / 2.0;
        
        self.mouse_position = physics2d_vec2(mouse_pos, pixels_per_unit);
        
        let left_mouse = sfml::window::mouse::Button::Left.is_pressed();
        let right_mouse = sfml::window::mouse::Button::Right.is_pressed();
        
        self.left_mouse_released = !self.left_mouse_pressed && left_mouse;
        self.right_mouse_released = !self.right_mouse_pressed && right_mouse;
        
        self.left_mouse_pressed = left_mouse;
        self.right_mouse_pressed = right_mouse;
    }
}
