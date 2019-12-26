#![warn(clippy::all)]

use std::num;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use std::time::{Duration, Instant};

mod vector;
use vector::Vec3;

mod camera;
use camera::Camera;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

#[allow(dead_code)]
fn main() -> Result<(), String> {
    println!("Hello, world!");

    // test vec2

    let cam = Camera::set(Vec3::zero(), Vec3(0.0, 0.0, -1.0), 0.8);

    println!("Camera: {:?}", cam);

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

    let mut isRendering: bool = false;

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
                            render_canvas_line(&canvas, &cam, y);

                            let next_y = y + 1;
                            if next_y < SCREEN_HEIGHT {
                                canvas.line(
                                    0,
                                    next_y as i16,
                                    SCREEN_WIDTH as i16,
                                    next_y as i16,
                                    pixels::Color::RGBA(255, 255, 255, 255),
                                )?;
                            }
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
    mut canvas: &sdl2::render::Canvas<sdl2::video::Window>,
    cam: &Camera,
    y: u32,
) {
    for x in 0..SCREEN_WIDTH {
        let r = cam.generate_ray(x as f32, y as f32);
        let color = raytrace(&r, 0);
        canvas.pixel(x as i16, y as i16, color);
    }
}

fn raytrace(ray: &Ray, mut depth: u16) -> pixels::Color {
    //let maxDepth: f32 = 200.0;
    /*let scale = ray.travelDistance / maxDepth;
    if scale > 1.0{
        scale = 1.0;
    }
    let scale2 = 1.0  -scale;*/
    pixels::Color::RGB(255, 0, 0)
}

pub struct Ray {
    is_intersected: bool, // todo: enum with intersection info
    direction: Vec3,
    origin: Vec3,
    travelDistance: f32,
}
