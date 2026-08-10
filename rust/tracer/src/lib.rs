use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub type Color = Vec3;
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
    pub fn unit(self) -> Self {
        self / self.length().max(1e-12)
    }
    pub fn near_zero(self) -> bool {
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }
    pub fn reflect(self, n: Self) -> Self {
        self - n * 2.0 * self.dot(n)
    }
    pub fn refract(self, n: Self, eta: f32) -> Self {
        let cos = (-self).dot(n).min(1.0);
        let r_out_perp = (self + n * cos) * eta;
        let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
    pub fn clamp(self, lo: f32, hi: f32) -> Self {
        Self::new(
            self.x.clamp(lo, hi),
            self.y.clamp(lo, hi),
            self.z.clamp(lo, hi),
        )
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self {
        self * (1.0 / t)
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rng {
    state: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x9e3779b97f4a7c15 } else { seed },
        }
    }
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 16) as u32
    }
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / (u32::MAX as f32 + 1.0)
    }
    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }
    pub fn in_unit_sphere(&mut self) -> Vec3 {
        loop {
            let p = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    pub fn in_unit_disk(&mut self) -> Vec3 {
        loop {
            let p = Vec3::new(self.range(-1.0, 1.0), self.range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, roughness: f32 },
    Dielectric { ior: f32 },
    Emissive { color: Color, strength: f32 },
}
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}
impl Hit {
    fn new(r: Ray, t: f32, outward: Vec3, m: Material) -> Self {
        let front = r.direction.dot(outward) < 0.0;
        Self {
            point: r.at(t),
            normal: if front { outward } else { -outward },
            t,
            front_face: front,
            material: m,
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let d = half_b * half_b - a * c;
        if d < 0.0 {
            return None;
        }
        let root = d.sqrt();
        let mut t = (-half_b - root) / a;
        if t < t_min || t > t_max {
            t = (-half_b + root) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }
        Some(Hit::new(
            r,
            t,
            (r.at(t) - self.center) / self.radius,
            self.material,
        ))
    }
}
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
    pub horizon: Color,
    pub zenith: Color,
}
impl Scene {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest = t_max;
        let mut found = None;
        for o in &self.objects {
            if let Some(h) = o.hit(r, t_min, closest) {
                closest = h.t;
                found = Some(h)
            }
        }
        found
    }
    pub fn classic() -> Self {
        let o: Vec<Box<dyn Hittable + Send + Sync>> = vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian {
                    albedo: Color::new(0.24, 0.18, 0.13),
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Dielectric { ior: 1.5 },
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Lambertian {
                    albedo: Color::new(0.07, 0.18, 0.3),
                },
            }),
            Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal {
                    albedo: Color::new(0.8, 0.42, 0.12),
                    roughness: 0.08,
                },
            }),
        ];
        Self {
            objects: o,
            horizon: Color::new(0.72, 0.4, 0.22),
            zenith: Color::new(0.04, 0.18, 0.62),
        }
    }
    pub fn lights() -> Self {
        let mut s = Self::classic();
        s.objects.push(Box::new(Sphere {
            center: Vec3::new(0.0, 1.3, -1.0),
            radius: 0.35,
            material: Material::Emissive {
                color: Color::new(1.0, 0.32, 0.08),
                strength: 5.0,
            },
        }));
        s.horizon = Color::new(0.035, 0.022, 0.018);
        s.zenith = Color::new(0.008, 0.012, 0.02);
        s
    }
}
#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f32,
}
#[derive(Clone, Copy)]
pub struct CameraSettings {
    pub fov: f32,
    pub orbit_radius: f32,
    pub focus_distance: f32,
    pub aperture: f32,
}
impl Camera {
    pub fn new(width: u32, height: u32, settings: CameraSettings, yaw: f32, pitch: f32) -> Self {
        let aspect = width as f32 / height.max(1) as f32;
        let theta = settings.fov * PI / 180.0;
        let h = (theta / 2.0).tan();
        let vh = 2.0 * h;
        let vw = aspect * vh;
        let target = Vec3::new(0.0, 0.15, -1.0);
        let origin = Vec3::new(
            yaw.cos() * pitch.cos() * settings.orbit_radius,
            pitch.sin() * settings.orbit_radius,
            yaw.sin() * pitch.cos() * settings.orbit_radius + target.z,
        );
        let w = (origin - target).unit();
        let u = Vec3::new(0.0, 1.0, 0.0).cross(w).unit();
        let v = w.cross(u);
        Self {
            origin,
            lower_left: origin
                - u * vw * settings.focus_distance / 2.0
                - v * vh * settings.focus_distance / 2.0
                - w * settings.focus_distance,
            horizontal: u * vw * settings.focus_distance,
            vertical: v * vh * settings.focus_distance,
            u,
            v,
            lens_radius: settings.aperture / 2.0,
        }
    }
    pub fn ray(&self, s: f32, t: f32, rng: &mut Rng) -> Ray {
        let lens_sample = rng.in_unit_disk();
        let d =
            self.u * lens_sample.x * self.lens_radius + self.v * lens_sample.y * self.lens_radius;
        Ray::new(
            self.origin + d,
            self.lower_left + self.horizontal * s + self.vertical * t - self.origin - d,
        )
    }
}
fn schlick(cos: f32, ior: f32) -> f32 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
fn dielectric_direction(
    unit: Vec3,
    normal: Vec3,
    front_face: bool,
    ior: f32,
    rng: &mut Rng,
) -> Vec3 {
    let refraction = if front_face { 1.0 / ior } else { ior };
    let cos = (-unit).dot(normal).min(1.0);
    let sin = (1.0 - cos * cos).sqrt();
    let cannot = refraction * sin > 1.0;
    if cannot || schlick(cos, refraction) > rng.next_f32() {
        unit.reflect(normal)
    } else {
        unit.refract(normal, refraction)
    }
}
fn radiance(r: Ray, scene: &Scene, rng: &mut Rng, depth: u32) -> (Color, u32) {
    if depth == 0 {
        return (Color::default(), 1);
    }
    if let Some(h) = scene.hit(r, 0.001, f32::INFINITY) {
        match h.material {
            Material::Emissive { color, strength } => return (color * strength, 1),
            Material::Lambertian { albedo } => {
                let mut d = h.normal + rng.in_unit_sphere().unit();
                if d.near_zero() {
                    d = h.normal
                }
                let (color, rays) = radiance(Ray::new(h.point, d), scene, rng, depth - 1);
                return (albedo * color, rays + 1);
            }
            Material::Metal { albedo, roughness } => {
                let d = r.direction.unit().reflect(h.normal) + rng.in_unit_sphere() * roughness;
                if d.dot(h.normal) <= 0.0 {
                    return (Color::default(), 1);
                }
                let (color, rays) = radiance(Ray::new(h.point, d), scene, rng, depth - 1);
                return (albedo * color, rays + 1);
            }
            Material::Dielectric { ior } => {
                let unit = r.direction.unit();
                let dir = dielectric_direction(unit, h.normal, h.front_face, ior, rng);
                let (color, rays) = radiance(Ray::new(h.point, dir), scene, rng, depth - 1);
                return (color, rays + 1);
            }
        }
    }
    let unit = r.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    (scene.horizon * (1.0 - t) + scene.zenith * t, 1)
}
pub const CAMERA_SETTINGS: CameraSettings = CameraSettings {
    fov: 55.0,
    orbit_radius: 4.5,
    focus_distance: 4.5,
    aperture: 0.025,
};
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub max_depth: u32,
    pub pixels: Vec<Color>,
    pub camera: Camera,
    pub scene: Scene,
    pub scene_id: u32,
    pub yaw: f32,
    pub pitch: f32,
    pub last_pass_rays: u64,
}
impl Renderer {
    pub fn new(width: u32, height: u32, scene_id: u32) -> Self {
        let mut r = Self {
            width,
            height,
            samples: 0,
            max_depth: 8,
            pixels: vec![Color::default(); (width * height) as usize],
            camera: Camera::new(width, height, CAMERA_SETTINGS, PI / 2.0, 0.12),
            scene: if scene_id == 1 {
                Scene::lights()
            } else {
                Scene::classic()
            },
            scene_id,
            yaw: PI / 2.0,
            pitch: 0.12,
            last_pass_rays: 0,
        };
        r.rebuild_camera();
        r
    }
    pub fn rebuild_camera(&mut self) {
        self.camera = Camera::new(
            self.width,
            self.height,
            CAMERA_SETTINGS,
            self.yaw,
            self.pitch,
        )
    }
    pub fn reset(&mut self) {
        self.samples = 0;
        self.last_pass_rays = 0;
        self.pixels.fill(Color::default())
    }
    pub fn set_orbit(&mut self, yaw: f32, pitch: f32) {
        self.yaw = yaw;
        self.pitch = pitch.clamp(-1.1, 1.1);
        self.rebuild_camera();
        self.reset()
    }
    pub fn render_pixel(&self, x: u32, y: u32, sample: u32, seed: u64) -> (Color, u32) {
        let i = (y * self.width + x) as u64;
        let mut rng = Rng::new(
            seed.wrapping_add(i.wrapping_mul(0x9e3779b97f4a7c15))
                .wrapping_add(sample as u64 * 7919),
        );
        let u = (x as f32 + rng.next_f32()) / (self.width - 1).max(1) as f32;
        let v = ((self.height - 1 - y) as f32 + rng.next_f32()) / (self.height - 1).max(1) as f32;
        radiance(
            self.camera.ray(u, v, &mut rng),
            &self.scene,
            &mut rng,
            self.max_depth,
        )
    }
    pub fn render_pass(&mut self, seed: u64) {
        let sample = self.samples;
        self.last_pass_rays = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.width + x) as usize;
                let (sample_color, rays) = self.render_pixel(x, y, sample, seed);
                self.pixels[i] += sample_color;
                self.last_pass_rays += rays as u64;
            }
        }
        self.samples += 1
    }
    pub fn write_rgba(&self, out: &mut Vec<u8>) {
        out.clear();
        out.reserve(self.pixels.len() * 4);
        for c in &self.pixels {
            let scale = 1.0 / self.samples.max(1) as f32;
            let radiance = *c * scale;
            let mapped = Color::new(
                radiance.x / (1.0 + radiance.x),
                radiance.y / (1.0 + radiance.y),
                radiance.z / (1.0 + radiance.z),
            );
            let g = mapped.clamp(0.0, 1.0);
            out.extend([
                (g.x.sqrt() * 255.999) as u8,
                (g.y.sqrt() * 255.999) as u8,
                (g.z.sqrt() * 255.999) as u8,
                255,
            ]);
        }
    }
    pub fn rgba(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.pixels.len() * 4);
        self.write_rgba(&mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_invariants() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.cross(a), Vec3::default());
        assert!((a.dot(a) - a.length_squared()).abs() < 1e-6);
        assert!((a.unit().length() - 1.0).abs() < 1e-6);
    }
    #[test]
    fn sphere_cases() {
        let s = Sphere {
            center: Vec3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            material: Material::Lambertian {
                albedo: Color::new(1.0, 1.0, 1.0),
            },
        };
        assert!(s
            .hit(
                Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0)),
                0.0,
                100.0
            )
            .is_some());
        assert!(s
            .hit(
                Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0)),
                0.0,
                100.0
            )
            .is_none());
        assert!(s
            .hit(
                Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0)),
                0.0,
                100.0
            )
            .is_some());
        let inside = Sphere {
            center: Vec3::default(),
            radius: 1.0,
            material: s.material,
        };
        assert!(
            inside
                .hit(
                    Ray::new(Vec3::default(), Vec3::new(1.0, 0.0, 0.0)),
                    0.0,
                    100.0
                )
                .unwrap()
                .t
                > 0.0
        );
    }
    #[test]
    fn refraction_and_tir() {
        let v = Vec3::new(0.0, 0.0, -1.0);
        let n = Vec3::new(0.0, 0.0, 1.0);
        assert!(v.refract(n, 1.0 / 1.5).z < 0.0);
        let grazing = Vec3::new(0.99, 0.0, -0.1).unit();
        let reflected = dielectric_direction(grazing, n, false, 1.5, &mut Rng::new(1));
        assert!(reflected.dot(n) > 0.0);
        assert!((schlick(1.0, 1.0 / 1.5) - 0.04).abs() < 1e-5);
        assert!(schlick(0.01, 1.0 / 1.5) > 0.94);
    }
    #[test]
    fn rng_deterministic() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..20 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }
    #[test]
    fn render_checksum() {
        let mut r = Renderer::new(12, 8, 0);
        r.render_pass(7);
        let rgba = r.rgba();
        let sum: u64 = rgba.iter().map(|v| *v as u64).sum();
        assert_eq!(sum, 51683);
    }
    #[test]
    fn rendered_pixels_are_finite_and_non_negative() {
        for scene in [0, 1] {
            let mut r = Renderer::new(16, 10, scene);
            r.render_pass(11);
            r.render_pass(11);
            assert!(r
                .pixels
                .iter()
                .all(|c| { [c.x, c.y, c.z].iter().all(|v| v.is_finite() && *v >= 0.0) }));
        }
    }
}
