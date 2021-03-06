use super::math::vector::Vec3;
use super::ray::Ray;
use super::scene;
use super::Camera;

extern crate rand;
use rand::prelude::*;

use std::time::Instant;

pub const MAX_RAY_DEPTH: u16 = 50;
pub const RAYS_PER_PIXEL: u16 = 16;

pub fn render_scene(scene: &scene::Scene, canvas: &mut [u8]) -> Result<(), String> {
    println!("Start rendering..");
    let now = Instant::now();
    render_to_canvas(&scene, canvas, &scene.camera)?;
    println!(
        "Finished rendering: {} Milliseconds",
        now.elapsed().as_millis()
    );
    Ok(())
}

fn render_to_canvas(scene: &scene::Scene, canvas: &mut [u8], cam: &Camera) -> Result<(), String> {
    let mut rng = rand::thread_rng();

    let mut index: usize = 0;
    for y in 0..super::SCREEN_HEIGHT {
        for x in 0..super::SCREEN_WIDTH {
            let mut pixel_color = Vec3::zero();
            for n_rp in 0..RAYS_PER_PIXEL {
                let rand_coord: (f64, f64) = rng.gen();
                let mut r = cam.generate_ray(x as f64 + rand_coord.0, y as f64 + rand_coord.1);
                pixel_color += raytrace(&scene, &mut r, 0);
            }
            let (r, g, b) = to_color(pixel_color, RAYS_PER_PIXEL);
            canvas[index] = r;
            canvas[index + 1] = g;
            canvas[index + 2] = b;
            index = index + 3;
        }
    }
    Ok(())
}

fn raytrace(scene: &scene::Scene, ray: &mut Ray, depth: u16) -> Vec3 {
    if depth >= MAX_RAY_DEPTH {
        return Vec3(0.0, 0.0, 0.0);
    }

    for it in scene.objects.iter() {
        it.intersect(ray, 0.001);
    }

    if let Some(hit) = &ray.is_intersected {
        //let target = hit.position + hit.normal + Vec3::rand_in_unit_sphere();
        //let target = hit.position + hit.normal + Vec3::rand_unit_vector();
        //let mut ray = Ray::new(hit.position, target - hit.position);
        //return raytrace(scene, &mut ray, depth + 1) * 0.5;
        if let Some((attenuation, mut scattered_ray)) = hit.material.scatter(ray) {
            return attenuation * raytrace(scene, &mut scattered_ray, depth + 1);
        }

        //let target = hit.position + Vec3::rand_in_hemispere(hit.normal);
        return Vec3(0.0, 0.0, 0.0);
        //return (hit.normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
    }

    let t = 0.5 * (ray.direction.1 + 1.0);
    Vec3::fill(1.0) * (1.0 - t) + (Vec3(0.5, 0.7, 1.0) * t)
}

// Returning rgb u8
fn to_color(vec: Vec3, samples: u16) -> (u8, u8, u8) {
    let scale = 1.0 / samples as f64;

    let _r = (scale * vec.0).sqrt();
    let _g = (scale * vec.1).sqrt();
    let _b = (scale * vec.2).sqrt();

    let _r = f64::clamp(_r, 0.0, 0.9999) * 256.0;
    let _g = f64::clamp(_g, 0.0, 0.9999) * 256.0;
    let _b = f64::clamp(_b, 0.0, 0.9999) * 256.0;

    (_r as u8, _g as u8, _b as u8)
}
