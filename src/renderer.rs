use super::math;
use super::math::vector::{Vec3, Vector};
use super::ray::{IntersectData, Ray};
use super::scene;
use super::Camera;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Instant;

pub const MAX_RAY_DEPTH: u16 = 8;

pub fn render_scene(scene: &scene::Scene, canvas: &mut Canvas<Window>) -> Result<(), String> {
    let now = Instant::now();

    canvas.clear();

    render_to_canvas(&scene, canvas, &scene.camera)?;

    canvas.present();
    println!(
        "Finished rendering: {} Milliseconds",
        now.elapsed().as_millis()
    );
    Ok(())
}

fn render_to_canvas(
    scene: &scene::Scene,
    canvas: &Canvas<Window>,
    cam: &Camera,
) -> Result<(), String> {
    for y in 0..super::SCREEN_HEIGHT {
        for x in 0..super::SCREEN_WIDTH {
            let mut r = cam.generate_ray(x as f64, y as f64);
            let color = raytrace(&scene, &mut r, 0);
            canvas.pixel(x as i16, y as i16, color.to_color())?;
        }
    }
    Ok(())
}

fn raytrace(scene: &scene::Scene, ray: &mut Ray, depth: u16) -> Vec3 {
    let mut _color = math::vector::Vec3(0.0, 0.0, 0.0);

    // if it above the ray depth
    if depth >= MAX_RAY_DEPTH {
        return _color;
    }

    // check for intersection
    for it in scene.objects.iter() {
        it.intersect(ray);
    }

    let mut shade = 1.0;

    if let IntersectData::Found {
        material: in_material,
        normal: in_normal,
    } = &ray.is_intersected
    {
        let mat_color = Vec3::from_color(in_material.color);
        let _point_intersect: Vec3 = ray.origin + ray.direction * ray.travel_distance;
        let mut intersected_lightrays = 0u8;
        for _light in scene.lights.iter() {
            // vector to point of intersection to light
            let _to_light = _light.position - _point_intersect;
            let to_light_normalized = Vec3::normalize(_to_light);

            // shoot ray to lights

            let shadow_ray = &mut Ray {
                origin: _point_intersect + (to_light_normalized * 0.005),
                direction: to_light_normalized,
                is_intersected: IntersectData::None,
                travel_distance: _to_light.length(),
            };

            for it in scene.objects.iter() {
                it.intersect(shadow_ray);

                match shadow_ray.is_intersected {
                    IntersectData::Found { .. } => {
                        intersected_lightrays += 1;
                    }
                    _ => {}
                }

                // if let IntersectData::Found { .. } = shadow_ray.is_intersected {
                //     intersected_lightrays += 1;
                // }
            }

            shade = 1.0 - (intersected_lightrays as f64 / scene.lights.len() as f64);

            let angle = Vec3::dot(*in_normal, to_light_normalized);
            if angle > 0.0 {
                let length = _to_light.length();
                let dist_an = _light.intensity / (length * length);
                _color += mat_color * dist_an * angle * shade;
            }
        }
    }
    _color
}
