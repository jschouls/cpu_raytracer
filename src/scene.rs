use super::light::Light;
use super::material::{Dielectric, Lambertian, Material, Metal};
use super::shape::{Plane, Shape, Sphere};
use super::Camera;
use super::Vec2;
use super::Vec3;

use std::rc::Rc;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

// Create the scene.
pub fn create_scene() -> Scene {
    // Materials

    // Reference counter because this can be shared with others and rays.
    let floor_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3(1.0, 1.0, 1.0),
    });

    let sphere_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vec3(0.1, 0.2, 0.5),
    });

    let metal_material: Rc<dyn Material> = Rc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.3));

    let dielectric_mat: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));

    Scene {
        objects: vec![
            Box::new(Plane::new(Vec3::up(), 0.5, &floor_material)),
            // Box::new(Plane::new(Vec3(0.0, 0.0, 1.0), 5.0, &back_material)),
            // Box::new(Plane::new(Vec3(1.0, 0.0, 0.0), 5.0, &back_material)),
            // Box::new(Plane::new(Vec3(-1.0, 0.0, 0.0), 5.0, &back_material)),
            Box::new(Sphere::new(Vec3(0.0, 0.0, -2.0), 0.5, &sphere_material)),
            Box::new(Sphere::new(Vec3(1.0, 0.0, -2.0), 0.5, &metal_material)),
            Box::new(Sphere::new(Vec3(-1.0, 0.0, -2.0), 0.5, &dielectric_mat)),
            Box::new(Sphere::new(Vec3(-1.0, 0.0, -2.0), -0.45, &dielectric_mat)),
        ],
        lights: vec![],
        camera: Camera::set(Vec3(0.0, 0.25, 1.0), Vec3(0.0, 0.0, -1.0), 0.5),
    }
}

/// Debug window
// Drawing rect
const DRAW_COORDS_MIN_XY: Vec2 = Vec2(-5.0, -5.0);
const DRAW_COORDS_MAX_XY: Vec2 = Vec2(5.0, 5.0);

// To 2d scaling
fn scale_and_to_point(_vec: Vec2) -> Point {
    let screen_width = super::SCREEN_WIDTH as f64;
    let screen_height = super::SCREEN_HEIGHT as f64;

    let totalx = DRAW_COORDS_MAX_XY.0 - DRAW_COORDS_MIN_XY.0;
    let totaly = DRAW_COORDS_MAX_XY.1 - DRAW_COORDS_MIN_XY.1;

    let _x = ((_vec.0 - DRAW_COORDS_MIN_XY.0) / totalx) * screen_width;

    let _y = ((_vec.1 - DRAW_COORDS_MIN_XY.1) / totaly) * screen_height;

    Point::new(_x as i32, _y as i32)
}

fn draw_axis(_canvas: &mut Canvas<Window>) -> Result<(), String> {
    // Draw axis
    let startpoint = Point::new(20, 20);
    let size = 60;
    _canvas.set_draw_color(Color::RGB(0, 0, 255));
    _canvas.draw_line(startpoint, Point::new(startpoint.x(), size))?;
    _canvas.set_draw_color(Color::RGB(255, 0, 0));
    _canvas.draw_line(startpoint, Point::new(size, startpoint.y()))?;

    Ok(())
}

pub fn draw_line(
    _canvas: &mut Canvas<Window>,
    _start: Vec2,
    _end: Vec2,
    _color: Color,
) -> Result<(), String> {
    let _a = scale_and_to_point(_start);
    let _b = scale_and_to_point(_end);

    _canvas.set_draw_color(_color);
    _canvas.draw_line(_a, _b)?;

    Ok(())
}

fn draw_camera(_camera: &Camera, _canvas: &mut Canvas<Window>) -> Result<(), String> {
    let dir_p0 = (_camera.vp.p[0] - _camera.position) * 100.0;
    let white = Color::RGB(255, 255, 255);
    let red = Color::RGB(255, 0, 0);
    draw_line(
        _canvas,
        Vec2(_camera.position.0, _camera.position.2),
        Vec2(dir_p0.0, dir_p0.2),
        white,
    )?;

    let dir_p1 = (_camera.vp.p[1] - _camera.position) * 100.0;

    draw_line(
        _canvas,
        Vec2(_camera.position.0, _camera.position.2),
        Vec2(dir_p1.0, dir_p1.2),
        white,
    )?;

    draw_line(
        _canvas,
        Vec2(_camera.vp.p[0].0, _camera.vp.p[0].2),
        Vec2(_camera.vp.p[1].0, _camera.vp.p[1].2),
        white,
    )?;

    let direction_endposition = _camera.position + (_camera.direction * _camera.vp.distance * 1.0);
    draw_line(
        _canvas,
        Vec2(_camera.position.0, _camera.position.2),
        Vec2(direction_endposition.0, direction_endposition.2),
        Color::RGB(0, 255, 0),
    )?;

    let r = Vec3::normalize(_camera.right);
    let right_end = _camera.position + (r * 1.0);
    draw_line(
        _canvas,
        Vec2(_camera.position.0, _camera.position.2),
        Vec2(right_end.0, right_end.2),
        Color::RGB(255, 0, 0),
    )?;

    Ok(())
}

pub fn draw_debug_scene(_scene: &Scene, _canvas: &mut Canvas<Window>) -> Result<(), String> {
    _canvas.set_draw_color(Color::RGB(0, 0, 0));
    _canvas.clear();

    draw_camera(&_scene.camera, _canvas)?;

    for it in _scene.objects.iter() {
        it.draw_debug(_canvas)?;
    }

    draw_axis(_canvas)?;
    _canvas.present();

    Ok(())
}
