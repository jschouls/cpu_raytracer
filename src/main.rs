#![warn(clippy::all)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;

mod vector;
use vector::Vec3;

mod camera;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
#[allow(dead_code)]
fn main() -> Result<(), String> {
    println!("Hello, world!");

    // test vec2

    let cam = camera::Camera::set(Vec3::zero(), Vec3(0.0, 0.0, -1.0), 0.8);

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

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::MouseButtonDown { x, y, .. } => {
                    let color = pixels::Color::RGBA(255, 255, 255, 255);
                    canvas.pixel(x as i16, y as i16, color)?;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    Ok(())
}
