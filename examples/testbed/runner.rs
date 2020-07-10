use crate::testbed::sfml::window::{VideoMode, Style, Event, ContextSettings};
use crate::testbed::sfml::graphics::{RenderWindow, Color, RenderTarget};
use crate::testbed::sfml::system::{Clock};

use super::Testbed;
use super::canvas::Canvas;
use super::input::Input;

pub fn run<T: Testbed>(mut testbed: T, config: super::config::Config) {
    let mut context_settings = ContextSettings::default();
    context_settings.antialiasing_level = 8;
    
    let mut window = RenderWindow::new(
        VideoMode::new(config.window_width, config.window_height, 32),
        config.title.as_ref(), Style::DEFAULT, &context_settings);
    
    window.set_framerate_limit(60);
    
    let mut canvas = Canvas::new(&config);
    let mut input = Input::new();
    
    let mut clock = Clock::start();
    
    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        input.clear();
        
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
            
            input.collect_event(event);
        }
        
        let dt = clock.restart().as_seconds();
        
        // Activate the window for OpenGL rendering
        window.set_active(true);
        
        // TODO: Clear color from config?
        window.clear(&Color::BLACK);
        
        input.collect(&window, config.pixels_per_unit);
        
        testbed.sfml_loop(&input, dt);
        
        testbed.sfml_draw(&mut canvas, dt);
        
        canvas.process_draw_queue(&mut window);
        
        // End the current frame and display its contents on screen
        window.display();
    }
}

