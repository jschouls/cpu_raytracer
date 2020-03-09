#![warn(clippy::all)]
extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Instant;

mod camera;
mod light;
mod material;
mod ray;
mod scene;
mod shape;
mod vector;

use camera::Camera;
use material::Material;
use ray::IntersectData;
use ray::Ray;
use vector::Vec2;
use vector::Vec3;
use vector::Vector;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
pub const MAX_RAY_DEPTH: u16 = 8;

#[allow(dead_code)]
fn main() -> Result<(), String> {
    println!("Hello, world!");

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

    let mut scene = scene::create_scene();

    // if feature debug screen is active
    // Debug windows, feature can be enabled or not. So canvas is an option
    let mut _debug_canvas = None;
    if cfg!(feature = "draw-debugger") {
        println!("Debugger constructing:");

        let debug_window = video_subsys
            .window("Debugger", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        _debug_canvas = Some(
            debug_window
                .into_canvas()
                .build()
                .map_err(|e| e.to_string())?,
        );

        scene::draw_debug_scene(&scene, _debug_canvas.as_mut().unwrap())?;
    }

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
                        scene.camera.position += scene.camera.direction * 0.05;
                    //render_scene(&scene, &mut canvas, &scene.camera);
                    } else if keycode == Keycode::S {
                        scene.camera.position += scene.camera.direction * -0.05;
                    //render_scene(&scene, &mut canvas, &scene.camera);
                    // } else if keycode == Keycode::A {
                    // } else if keycode == Keycode::D {
                    } else if keycode == Keycode::Space {
                    }

                    render_scene(&scene, &mut canvas, &scene.camera);
                    if render_debug_screen {
                        scene::draw_debug_scene(&scene, _debug_canvas.as_mut().unwrap())?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn render_scene(scene: &scene::Scene, canvas: &mut Canvas<Window>, cam: &Camera) {
    canvas.clear();
    let now = Instant::now();
    for y in 0..SCREEN_HEIGHT {
        render_canvas_line(&scene, &canvas, &cam, y);
    }
    canvas.present();
    println!("Finished rendering: {}", now.elapsed().as_millis());
}

fn render_canvas_line(scene: &scene::Scene, canvas: &Canvas<Window>, cam: &Camera, y: u32) {
    for x in 0..SCREEN_WIDTH {
        let mut r = cam.generate_ray(x as f32, y as f32);
        let color = raytrace(&scene, &mut r, 0);
        canvas.pixel(x as i16, y as i16, color);
    }
}

fn raytrace(scene: &scene::Scene, ray: &mut Ray, depth: u16) -> Color {
    let mut _color = Vec3(0.0, 0.0, 0.0);

    // if it above the ray depth
    if depth >= MAX_RAY_DEPTH {
        return _color.to_color();
    }

    // check for intersection
    for it in scene.objects.iter() {
        it.intersect(ray);
    }

    if let IntersectData::Found {
        material: in_material,
        normal: in_normal,
    } = &ray.is_intersected
    {
        let mat_color = Vec3::from_color(in_material.color);
        let _point_intersect: Vec3 = ray.origin + ray.direction * ray.travel_distance;
        for _light in scene.lights.iter() {
            // vector to point of intersection to light
            let _to_light = _light.position - _point_intersect;
            let _n_to_light = Vec3::normalize(_to_light);

            let angle = Vec3::dot(*in_normal, _n_to_light);
            if angle > 0.0 {
                let length = _to_light.length();
                let dist_an = _light.intensity / (length * length);
                _color += mat_color * dist_an * angle;
            }
        }
    }
    _color.to_color()
}
