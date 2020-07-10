use physics2d::*;

use crate::testbed::sfml;
use crate::testbed::sfml::window::Event;
use crate::testbed::sfml::graphics::RenderTarget;

use crate::testbed::physics2d_vec2;

pub use crate::testbed::sfml::window::Key as Key;

pub struct Input {
    pub has_focus: bool,
    
    pub mouse_position: Vec2,
    
    pub left_mouse_pressed: bool,
    pub left_mouse_released: bool,
    
    pub right_mouse_pressed: bool,
    pub right_mouse_released: bool,
    
    pub pressed_keys: Vec<Key>,
}

impl Input {
    pub(super) fn new() -> Input {
        Input {
            has_focus: false,
            mouse_position: Vec2::ZERO,
            left_mouse_pressed: false,
            left_mouse_released: false,
            right_mouse_pressed: false,
            right_mouse_released: false,
            pressed_keys: Vec::new(),
        }
    }
    
    pub(super) fn clear(&mut self) {
        self.pressed_keys.clear();
    }
    
    pub(super) fn collect_event(&mut self, event: Event) {
        match event {
            Event::GainedFocus => self.has_focus = true,
            Event::LostFocus => self.has_focus = false,
            Event::KeyPressed { code, alt: _, ctrl: _, shift: _, system: _ } => {
                self.pressed_keys.push(code);
            }
            _ => {}
        }
    }
    
    pub(super) fn collect(&mut self, window: &sfml::graphics::RenderWindow, pixels_per_unit: f32) {
        self.collect_mouse_input(window, pixels_per_unit);
    }
    
    fn collect_mouse_input(&mut self, window: &sfml::graphics::RenderWindow, pixels_per_unit: f32) {
        let mouse_pos = window.mouse_position();
        
        // Register mouse input only if mouse is inside window
        if mouse_pos.x < 0 || mouse_pos.x > window.view().size().x as i32 {
            return;
        }
    
        if mouse_pos.y < 0 || mouse_pos.y > window.view().size().y as i32 {
            return;
        }
        
        let mut mouse_pos = sfml::system::Vector2f::new(mouse_pos.x as f32, mouse_pos.y as f32);
        mouse_pos -= window.view().center() + window.view().size() / 2.0;
        
        self.mouse_position = physics2d_vec2(mouse_pos, pixels_per_unit);
        
        // Register mouse clicks only if the window is focused and click is done inside window
        let left_mouse = sfml::window::mouse::Button::Left.is_pressed() && self.has_focus;
        let right_mouse = sfml::window::mouse::Button::Right.is_pressed() && self.has_focus;
        
        self.left_mouse_released = !self.left_mouse_pressed && left_mouse;
        self.right_mouse_released = !self.right_mouse_pressed && right_mouse;
        
        self.left_mouse_pressed = left_mouse;
        self.right_mouse_pressed = right_mouse;
    }
}
