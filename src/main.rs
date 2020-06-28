#![warn(clippy::all)]
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod camera;
mod light;
mod material;
mod math;
mod ray;
mod renderer;
mod scene;
mod shape;

use camera::Camera;
use math::vector::Vec3;
//use ray::{ IntersectData, ;
//use ray::Ray;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

#[allow(dead_code)]
fn main() -> Result<(), String> {
    let mut render_on_change = false;

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("CPU Raytracer", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump()?;

    let scene = scene::create_scene();

    // if feature debug screen is active
    // Debug windows, feature can be enabled or not. So canvas is an option
    //let mut _debug_canvas = None;
    let render_debug_screen = cfg!(feature = "draw-debugger");

    //if render_debug_screen {
    //    println!("Debugger constructing:");

    //    let debug_window = video_subsys
    //        .window("Debugger", SCREEN_WIDTH, SCREEN_HEIGHT)
    //        .position_centered()
    //        .opengl()
    //        .build()
    //        .map_err(|e| e.to_string())?;

    //    _debug_canvas = Some(
    //        debug_window
    //            .into_canvas()
    //            .build()
    //            .map_err(|e| e.to_string())?,
    //    );

    //    //scene::draw_debug_scene(&scene, _debug_canvas.as_mut().unwrap())?;
    //}

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
                    } else if keycode == Keycode::W {
                        // scene.camera.move_direction(scene.camera.direction * 0.05);
                    } else if keycode == Keycode::S {
                        // scene.camera.move_direction(scene.camera.direction * -0.05);
                    } else if keycode == Keycode::A {
                        // scene.camera.move_direction(scene.camera.right * 0.05);
                    } else if keycode == Keycode::D {
                        // scene.camera.move_direction(scene.camera.right * -0.05);
                    } else if keycode == Keycode::Space {
                        // Toggle if you want to render on a change(event) in scene.
                        render_on_change = !render_on_change;
                    }

                    if render_on_change {
                        renderer::render_scene(&scene, &mut canvas)?;
                    }
                    if render_debug_screen {
                        //scene::draw_debug_scene(&scene, _debug_canvas.as_mut().unwrap())?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
