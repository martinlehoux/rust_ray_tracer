use std::{error, fs, io, ops};

fn convert_to_u8(taint: f64) -> u8 {
    (taint * u8::MAX as f64) as u8
}

fn mix_taint(taint_1: f64, taint_2: f64, ratio: f64) -> f64 {
    taint_1 * ratio + taint_2 * (1.0 - ratio)
}

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

impl Color {
    fn draw(&self, writer: &mut (dyn io::Write)) -> Result<(), Box<dyn error::Error>> {
        writer.write(convert_to_u8(self.red).to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(convert_to_u8(self.green).to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(convert_to_u8(self.blue).to_string().as_bytes())?;
        writer.write(b"\n")?;
        Ok(())
    }

    fn mix(color_1: Color, color_2: Color, ratio: f64) -> Color {
        Color {
            red: mix_taint(color_1.red, color_2.red, ratio),
            green: mix_taint(color_1.green, color_2.green, ratio),
            blue: mix_taint(color_1.blue, color_2.blue, ratio),
        }
    }
}

const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
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

    fn sample() -> Image {
        let mut image = Image {
            width: 1920,
            height: 1080,
            lines: vec![],
        };
        for y in 0..image.height {
            let mut line = Vec::<Color>::new();
            for x in 0..image.width {
                line.push(Color {
                    red: (x as f64 / (image.width - 1) as f64),
                    green: (y as f64 / (image.height - 1) as f64),
                    blue: 0.25,
                })
            }
            image.lines.push(line);
        }
        image
    }
}

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
    type Output = Vec3;
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
    type Output = Vec3;
    fn bitand(self, vec: Self) -> Self::Output {
        Vec3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        }
    }
}

// Dot product
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * vec.x,
            y: self.y * vec.y,
            z: self.z * vec.z,
        }
    }
}

impl Vec3 {
    fn square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn norm(&self) -> f64 {
        self.square().sqrt()
    }

    fn unit(self) -> Vec3 {
        let norm = self.norm();
        self * (1 as f64 / norm)
    }
}

type Point3 = Vec3;
type Time = f64;

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(self, t: Time) -> Point3 {
        self.origin + self.direction * t
    }
}

fn main() {
    println!("Hello, world!");

    let image = Image::sample();
    let file = fs::File::create("test.ppm").unwrap();
    let mut buffer = io::BufWriter::new(file);
    image.draw(&mut buffer).unwrap();
}
