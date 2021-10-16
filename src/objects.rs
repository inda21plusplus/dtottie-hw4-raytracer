use crate::coordinate::Coordinate;
use crate::scene::Scene;
use macaw::Vec3;

pub struct Hit<'a> {
    pub distance: f32,
    pub shape: &'a Shape,
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Color,
    pub strength: f32,
}

pub struct SphericalLight {
    pub position: Coordinate,
    pub color: Color,
    pub strength: f32,
}

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Coordinate,
    pub direction: Vec3,
}

#[derive(Clone, Copy)]
pub enum Shape {
    Plane(Plane),
    Sphere(Sphere),
}

//private structs

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Coordinate,
    radius: f32,
    color: Color,
    irradiance: f32,
    reflectivity: f64,
}
#[derive(Clone, Copy)]
pub struct Plane {
    center: Coordinate,
    normal: Vec3,
    color: Color,
    irradiance: f32,
    reflectivity: f64,
}

//traits
pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f32>;
    fn surface_normal(&self, intersection_spot: &Coordinate) -> Vec3;
}

//impl blocks
impl Shape {
    pub fn new_sphere(
        center: Coordinate,
        radius: f32,
        color: Color,
        irradiance: f32,
        reflectivity: f64,
    ) -> Shape {
        Shape::Sphere(Sphere {
            center,
            radius,
            color,
            irradiance,
            reflectivity,
        })
    }
    pub fn new_plane(
        center: Coordinate,
        normal: Vec3,
        color: Color,
        irradiance: f32,
        reflectivity: f64,
    ) -> Shape {
        Shape::Plane(Plane {
            center,
            normal,
            color,
            irradiance,
            reflectivity,
        })
    }
    pub fn color(&self) -> Color {
        match self {
            Shape::Sphere(sphere) => sphere.color,
            Shape::Plane(plane) => plane.color,
        }
    }
    pub fn irradiance(&self) -> f32 {
        match *self {
            Shape::Sphere(sphere) => sphere.irradiance,
            Shape::Plane(plane) => plane.irradiance,
        }
    }
    pub fn reflectivity(&self) -> f64 {
        match *self {
            Shape::Sphere(sphere) => sphere.reflectivity,
            Shape::Plane(plane) => plane.reflectivity,
        }
    }
}
impl Ray {
    pub fn first(x: u32, y: u32, scene: &Scene) -> Self {
        let fov = (scene.fov.to_radians() / 2.0).tan();
        let aspect = (scene.width as f32) / (scene.height as f32);
        let scanner_x = ((((x as f32 + 0.5f32) / scene.width as f32) * 2f32 - 1f32) * aspect) * fov;
        let scanner_y = (1f32 - ((y as f32 + 0.5f32) / scene.height as f32) * 2f32) * fov;

        Self {
            origin: Coordinate {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            direction: Vec3::new(scanner_x, scanner_y, -1.0).normalize(),
        }
    }
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        match *self {
            Shape::Plane(plane) => plane.hit(ray),
            Shape::Sphere(sphere) => sphere.hit(ray),
        }
    }
    fn surface_normal(&self, intersection_spot: &Coordinate) -> Vec3 {
        match *self {
            Shape::Plane(ref plane) => plane.surface_normal(intersection_spot),
            Shape::Sphere(ref sphere) => sphere.surface_normal(intersection_spot),
        }
    }
}

//https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection
impl Hittable for Plane {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn surface_normal(&self, _intersection_spot: &Coordinate) -> Vec3 {
        -self.normal.normalize()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f32> {
        let center_line: Vec3 = self.center - ray.origin;
        let length = center_line.dot(ray.direction);
        let squared = (center_line.length() * center_line.length()) - (length * length);

        if squared > (self.radius * self.radius) {
            return None;
        }
        let width_square = ((self.radius * self.radius) - squared).sqrt();
        let (neg, pos) = (length - width_square, length + width_square);
        if neg < 0.0 && pos < 0.0 {
            return None;
        }

        let distance = if neg < pos { neg } else { pos };
        Option::Some(distance)
    }
    fn surface_normal(&self, intersection_spot: &Coordinate) -> Vec3 {
        (*intersection_spot - self.center).normalize()
    }
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => d.color,
            Light::Spherical(ref s) => s.color,
        }
    }
    //https://bheisler.github.io/post/writing-raytracer-in-rust-part-3/
    pub fn direction_from(&self, hit_Coordinate: &Coordinate) -> Vec3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (s.position - *hit_Coordinate).normalize(),
        }
    }
    //https://bheisler.github.io/post/writing-raytracer-in-rust-part-3/
    pub fn strength(&self, hit_Coordinate: &Coordinate) -> f32 {
        match *self {
            Light::Directional(ref d) => d.strength,
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_Coordinate).dot(s.position - *hit_Coordinate) as f32;
                s.strength / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }
    //https://bheisler.github.io/post/writing-raytracer-in-rust-part-3/
    pub fn distance(&self, hit_Coordinate: &Coordinate) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_Coordinate).length() as f64,
        }
    }
}

impl Color {
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        }
    }
}

impl<'a> Hit<'a> {
    pub fn new<'b>(distance: f32, shape: &'b Shape) -> Hit<'b> {
        if !distance.is_finite() {
            panic!("Intersection must have a finite distance.");
        }
        Hit { distance, shape }
    }
}

//https://stackoverflow.com/questions/39169795/error-when-using-operators-with-a-generic-type
impl std::ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

//https://stackoverflow.com/questions/39169795/error-when-using-operators-with-a-generic-type
impl std::ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        Color {
            r: (self.r as f64 * rhs),
            g: (self.g as f64 * rhs),
            b: (self.b as f64 * rhs),
        }
    }
}

//https://stackoverflow.com/questions/39169795/error-when-using-operators-with-a-generic-type
impl std::ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            b: self.b + other.b,
            g: self.g + other.g,
        }
    }
}
