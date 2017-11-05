
use super::Testbed as Testbed;

use sfml::window::{VideoMode, Style, Event};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Shape, CircleShape, Transformable};
use sfml::system::{Vector2f};


pub fn run<T: Testbed>(testbed: &mut T) {
    let config = testbed.config();
    let mut window = RenderWindow::new(
        VideoMode::new(config.window_width, config.window_height, 32),
                                       config.title, Style::DEFAULT, &Default::default());
    
    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
        }
        
        // Activate the window for OpenGL rendering
        window.set_active(true);
        
        // OpenGL drawing commands go here...
        window.clear(&Color::BLACK);
        
        window.draw(&circle);
        
        // End the current frame and display its contents on screen
        window.display();
    }
}

