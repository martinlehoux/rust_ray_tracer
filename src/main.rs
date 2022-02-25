use std::{error, fs, io, ops};

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn draw(&self, writer: &mut (dyn io::Write)) -> Result<(), Box<dyn error::Error>> {
        writer.write(self.red.to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(self.green.to_string().as_bytes())?;
        writer.write(b" ")?;
        writer.write(self.blue.to_string().as_bytes())?;
        writer.write(b"\n")?;
        Ok(())
    }
}

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
                    red: (x as f64 / (image.width - 1) as f64 * 255 as f64) as u8,
                    green: (y as f64 / (image.height - 1) as f64 * 255 as f64) as u8,
                    blue: 64,
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
