extern crate serde_json;
extern crate glam;

use std::{fs, ops::{Sub, Mul}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        let r: f32 = (self.r as f32 * rhs).round().clamp(0.0, 255.0);
        let g: f32 = (self.g as f32 * rhs).round().clamp(0.0, 255.0);
        let b: f32 = (self.b as f32 * rhs).round().clamp(0.0, 255.0);

        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) struct Vec3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Into<glam::Vec3> for Vec3D {
    fn into(self) -> glam::Vec3 {
        glam::Vec3 { x: self.x, y: self.y, z: self.z }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Sub<glam::Vec3> for Point3D {
    fn sub(self, rhs: glam::Vec3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        glam::vec3(x, y, z)
    }

    type Output = glam::Vec3;
}

impl Sub<Point3D> for glam::Vec3 {
    fn sub(self, rhs: Point3D) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        glam::vec3(x, y, z)
    }

    type Output = glam::Vec3;
}

impl Into<(f32, f32, f32)> for Point3D {
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Viewport {
    width: f32,
    height: f32,
    distance: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Sphere {
    pub position: Point3D,
    pub radius: f32,
    pub specular: i32,
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum LightTypes {
    Ambient,
    Point,
    Directional,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Light {
    pub t: LightTypes,
    pub intensity: f32,
    pub position: Option<Point3D>,
    pub direction: Option<Vec3D>
       
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Scene {
    background: Color,
    origin: Point3D,
    viewport: Viewport,
    orientation: Vec3D,
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn load(path: &str) -> Self {
        let scene_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => panic!("{}", e),
        };

        let res = match serde_json::from_str(&scene_content) {
            Ok(scene) => scene,
            Err(e) => panic!("{}", e),
        };

        res
    }

    pub(crate) fn get_background(&self) -> (u8, u8, u8) {
        self.background.into()
    }

    pub(crate) fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub(crate) fn get_spheres(&self) -> &Vec<Sphere> {
        &self.spheres
    }

    pub(crate) fn get_origin(&self) -> (f32, f32, f32) {
        self.origin.into()
    }

    pub(crate) fn get_viewport_info(&self) -> (f32, f32, f32) {
        (self.viewport.width, self.viewport.height, self.viewport.distance)
    }
}
