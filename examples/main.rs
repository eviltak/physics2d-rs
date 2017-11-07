extern crate sfml;
extern crate physics2d;

mod testbed;

struct MainTestbed;

impl testbed::Testbed for MainTestbed {
    fn sfml_loop(&mut self) {
    
    }
    
    fn sfml_draw(&mut self, canvas: &mut testbed::Canvas) {
        canvas.draw_circle(physics2d::math::Vec2::up() * 25.0, 5.0);
    }
}

fn main() {
    let config = testbed::Config {
        title: "Main".to_string(),
        window_width: 800,
        window_height: 600,
        pixels_per_unit: 10.0,
    };
    
    let mut testbed = MainTestbed;
    
    testbed::run(testbed, config);
}
