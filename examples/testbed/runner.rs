
use sfml::window::{VideoMode, Style, Event};
use sfml::graphics::{RenderWindow, RenderTarget, Color,};
use sfml::system::Clock;

use super::Testbed;
use super::canvas::Canvas;

pub fn run<T: Testbed>(mut testbed: T, config: super::config::Config) {
    let mut window = RenderWindow::new(
        VideoMode::new(config.window_width, config.window_height, 32),
                                       config.title.as_ref(), Style::DEFAULT, &Default::default());
    
    let mut canvas = Canvas::new(&config);
    
    let mut clock = Clock::start();
    
    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        // Event processing
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
        }
        
        let dt = clock.restart().as_seconds();
        
        // Activate the window for OpenGL rendering
        window.set_active(true);
        
        // TODO: Clear color from config?
        window.clear(&Color::BLACK);
        
        testbed.sfml_loop(dt);
        
        testbed.sfml_draw(&mut canvas, dt);
        
        canvas.draw_queue_to_window(&mut window);
        
        // End the current frame and display its contents on screen
        window.display();
    }
}

