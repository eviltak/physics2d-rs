#![feature(test)]

extern crate test;
extern crate physics2d;

use test::Bencher;

use physics2d::*;

fn box_vertices(w: f32, h: f32) -> Vec<Vec2> {
    vec![Vec2::ZERO, Vec2::RIGHT * w, Vec2::new(w, h), Vec2::UP * h]
}

fn initialize(pyramid_base_count: u32) -> World {
    let mut world = World::default();
    
    let ground_width = pyramid_base_count as f32 * 1.5;
    let ground_height = 1.0;
    let ground_poly = shapes::Polygon::new(box_vertices(ground_width, ground_height));
    
    let mut ground = Body::new(ground_poly.into_shape(), 10.0, Material::new(0.4, 0.4));
    
    ground.set_static();
    world.add_body(ground);
    
    const WIDTH: f32 = 1.0;
    let square = shapes::Polygon::new(box_vertices(WIDTH, WIDTH));
    
    let mut x = Vec2::new(WIDTH * 0.5, WIDTH * 0.5 + ground_height);
    
    for i in 0..pyramid_base_count {
        let mut y = x;
        
        for _j in i..pyramid_base_count {
            let mut body = Body::new(square.clone().into_shape(), 10.0, Material::new(0.3, 0.3));
            body.transform.position = y;
            world.add_body(body);
            y += Vec2::RIGHT * WIDTH * 1.125;
        }
        
        x += Vec2::new(0.5625, 1.0) * WIDTH;
    }
    
    world
}

const DT: f32 = 1.0 / 60.0;

fn warmup(world: &mut World) {
    const WARMUP_ITERATIONS: u32 = 120;
    
    for _i in 0..WARMUP_ITERATIONS {
        world.update(DT);
    }
}

fn bench_pyramid(pyramid_base_count: u32, b: &mut Bencher) {
    let mut world = initialize(pyramid_base_count);
    warmup(&mut world);
    b.iter(|| world.update(DT));
}

#[bench]
fn pyramid_base_10(b: &mut Bencher) {
    bench_pyramid(10, b);
}

#[bench]
fn pyramid_base_20(b: &mut Bencher) {
    bench_pyramid(20, b);
}

#[bench]
fn pyramid_base_30(b: &mut Bencher) {
    bench_pyramid(30, b);
}

#[bench]
fn pyramid_base_40(b: &mut Bencher) {
    bench_pyramid(40, b);
}

#[bench]
fn pyramid_base_50(b: &mut Bencher) {
    bench_pyramid(50, b);
}

#[bench]
fn pyramid_base_60(b: &mut Bencher) {
    bench_pyramid(60, b);
}
