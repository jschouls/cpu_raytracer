#![warn(clippy::all)]

use std::num;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::rc::Rc;
use std::time::{Duration, Instant};

mod camera;
mod material;
mod ray;
mod shape;
mod vector;

use camera::Camera;
use material::Material;
use ray::IntersectData;
use ray::Ray;
use shape::{Plane, Shape, Sphere};
use vector::Vec3;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub const MAX_RAY_DEPTH: u16 = 8;

#[allow(dead_code)]
fn main() -> Result<(), String> {
    println!("Hello, world!");

    // test vec2

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("CPU Raytracer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    let mut is_rendering: bool = false;

    // Create scene
    let cam = Camera::set(Vec3::zero(), Vec3(0.0, 0.0, -1.0), 0.8);

    // Reference counter because this can be shared with others and rays.
    let floorMaterial = Rc::new(Material {
        color: pixels::Color::RGB(255, 255, 255),
        reflection: 0.0,
        refraction: 0.0,
    });

    //let floor = ;

    let scene: Vec<Box<dyn Shape>> = vec![
        Box::new(shape::Plane::new(Vec3::up(), 0.5, &floorMaterial)),
        //Box::new(shape::Plane::new(Vec3(0.0, 0.0, 1.0), 20.0, &floorMaterial)),
    ];

    println!("floor ref count: {:?}", Rc::strong_count(&floorMaterial));

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        canvas.clear();
                        let now = Instant::now();
                        for y in 0..SCREEN_HEIGHT {
                            render_canvas_line(&scene, &canvas, &cam, y);
                        }
                        canvas.present();
                        println!("Finished rendering: {}", now.elapsed().as_millis());
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn render_canvas_line(
    scene: &Vec<Box<dyn Shape>>,
    mut canvas: &sdl2::render::Canvas<sdl2::video::Window>,
    cam: &Camera,
    y: u32,
) {
    for x in 0..SCREEN_WIDTH {
        //let r : &
        let mut r = cam.generate_ray(x as f32, y as f32);
        let color = raytrace(&scene, &mut r, 0);
        canvas.pixel(x as i16, y as i16, color);
    }
}

fn raytrace(scene: &Vec<Box<dyn Shape>>, ray: &mut Ray, mut depth: u16) -> pixels::Color {
    let _color = pixels::Color::RGB(0, 0, 0);

    // if it above the ray depth
    if depth >= MAX_RAY_DEPTH {
        return _color;
    }

    // check for intersection
    for it in scene.iter() {
        it.intersect(ray);
    }

    // is intersected? return material color for now.
    match &ray.is_intersected {
        IntersectData::Found { material, .. } => material.color,
        _ => _color,
    }
}
