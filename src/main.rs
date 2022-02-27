use std::{
    error, fs,
    io::{self, Write},
    ops, rc,
    str::MatchIndices,
};

use rand::{self, Rng};

fn convert_to_u8(taint: f64) -> u8 {
    (taint.sqrt() * u8::MAX as f64) as u8
}

#[derive(Clone, Copy)]
struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, coef: f64) -> Self::Output {
        Color {
            red: self.red * coef,
            green: self.green * coef,
            blue: self.blue * coef,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, color: Color) -> Self::Output {
        Color {
            red: self.red * color.red,
            green: self.green * color.green,
            blue: self.blue * color.blue,
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, color: Color) -> Self::Output {
        Color {
            red: self.red + color.red,
            green: self.green + color.green,
            blue: self.blue + color.blue,
        }
    }
}

impl Color {
    fn draw(self, writer: &mut (dyn io::Write)) -> Result<(), Box<dyn error::Error>> {
        writer.write(convert_to_u8(self.red).to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(convert_to_u8(self.green).to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(convert_to_u8(self.blue).to_string().as_bytes())?;
        writer.write(b"\n")?;
        Ok(())
    }

    fn blend(color_1: Color, color_2: Color, ratio: f64) -> Color {
        color_1 * ratio + color_2 * (1.0 - ratio)
    }

    fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }
}

const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};
const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};
const BLUE_SKY: Color = Color {
    red: 0.5,
    green: 0.7,
    blue: 1.0,
};
const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};
const COPPER: Color = Color {
    red: 0.722,
    green: 0.451,
    blue: 0.20,
};

struct Image {
    width: usize,
    height: usize,
    lines: Vec<Vec<Color>>,
}

impl Image {
    fn draw(&self, writer: &mut (dyn io::Write + 'static)) -> Result<(), Box<dyn error::Error>> {
        writer.write(b"P3\n")?;
        writer.write(self.width.to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(self.height.to_string().as_bytes())?;
        writer.write(b"\n255\n")?;
        for line in (&self.lines).into_iter().rev() {
            for pixel in line {
                pixel.draw(writer)?;
            }
        }
        Ok(())
    }

    fn sample(
        height: usize,
        width: usize,
        camera: &Camera,
        world: &World,
        sampling: usize,
        depth: usize,
    ) -> Image {
        let mut rng = rand::thread_rng();
        let mut image = Image {
            width,
            height,
            lines: vec![],
        };
        for y in 0..image.height {
            print!("\rRendering line {} of {height}", y + 1);
            io::stdout().flush().unwrap();
            let mut line = Vec::<Color>::new();
            for x in 0..image.width {
                let mut color = BLACK;
                for _ in 0..sampling {
                    let u = (x as f64 + rng.gen_range(0.0..1.0)) / (image.width - 1) as f64;
                    let v = (y as f64 + rng.gen_range(0.0..1.0)) / (image.height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    color = color + Ray::color(ray, world, depth);
                }
                line.push(color * (1.0 / sampling as f64));
            }
            image.lines.push(line);
        }
        image
    }
}

#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, vec: Self) -> Self::Output {
        Vec3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, vec: Self) -> Self::Output {
        Vec3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, coef: f64) -> Self::Output {
        Vec3 {
            x: self.x * coef,
            y: self.y * coef,
            z: self.z * coef,
        }
    }
}

// Cross product
impl ops::BitAnd for Vec3 {
    type Output = Self;
    fn bitand(self, vec: Vec3) -> Self::Output {
        Vec3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        }
    }
}

// Dot product
impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, vec: Vec3) -> Self::Output {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn norm(self) -> f64 {
        (self * self).sqrt()
    }

    fn unit(self) -> Vec3 {
        let norm = self.norm();
        self * (1 as f64 / norm)
    }

    fn random() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Vec3::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            );
            if vec * vec <= 1.0 {
                return vec;
            }
        }
    }

    fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * (self * normal) * 2.0
    }
}

type Point3 = Vec3;

#[derive(Clone, Copy)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: f64) -> Point3 {
        self.origin + (self.direction * t)
    }

    fn color(ray: Ray, hittable: &dyn Hittable, depth: usize) -> Color {
        if depth == 0 {
            return BLACK;
        };
        match hittable.hit(&ray, 0.001, f64::MAX) {
            None => Color::blend(BLUE_SKY, WHITE, 0.5 * (ray.direction.unit().y + 1.0)),
            Some(hit) => match hit.material.scatter(&ray, &hit) {
                Some((attenuation, scattered)) => {
                    attenuation * Ray::color(scattered, hittable, depth - 1)
                }
                None => BLACK,
            },
        }
    }
}

struct Sphere {
    center: Point3,
    radius: f64,
    material: rc::Rc<dyn Material>,
}

struct HitRecord {
    point: Point3,
    normal: Vec3,
    time: f64,
    material: rc::Rc<dyn Material>,
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let half_b = oc * ray.direction;
        let c = (oc * oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut solutions = [(-half_b - sqrtd) / a, (-half_b + sqrtd) / a]
                .into_iter()
                .filter(|time| *time >= t_min && *time <= t_max);
            match solutions.next() {
                None => None,
                Some(time) => Some(HitRecord {
                    time,
                    point: ray.at(time),
                    normal: (ray.at(time) - self.center) * (1.0 / self.radius),
                    material: self.material.clone(),
                }),
            }
        }
    }
}

struct World(Vec<Box<dyn Hittable>>);

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        (&self.0)
            .into_iter()
            .find_map(|hittable| hittable.hit(ray, t_min, t_max))
    }
}

struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal * 0.5
                - vertical * 0.5
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}

trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

struct Matte {
    albedo: Color,
}

impl Material for Matte {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let direction = hit.normal + Vec3::random().unit();
        // TODO: Check not null
        let scattered = Ray {
            origin: hit.point,
            direction,
        };
        Some((self.albedo, scattered))
    }
}

struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction.unit().reflect(hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + Vec3::random().unit() * self.fuzziness,
        };
        if scattered.direction * hit.normal > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
        // TODO: Check direction
    }
}

fn main() {
    println!("Hello, world!");

    let sampling = 100;
    let depth = 50;
    let camera = Camera::new(16.0 / 9.0, 2.0, 1.0);

    let ground = rc::Rc::new(Matte {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let matte = rc::Rc::new(Matte {
        albedo: Color::new(0.8, 0.6, 0.2),
    });
    let metal = rc::Rc::new(Metal {
        albedo: COPPER,
        fuzziness: 0.5,
    });

    let world = World(vec![
        Box::new(Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: matte.clone(),
        }),
        Box::new(Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: metal.clone(),
        }),
        Box::new(Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: ground.clone(),
        }),
    ]);

    let image = Image::sample(1080, 1920, &camera, &world, sampling, depth);
    let file = fs::File::create("test.ppm").unwrap();
    let mut buffer = io::BufWriter::new(file);
    image.draw(&mut buffer).unwrap();
}
