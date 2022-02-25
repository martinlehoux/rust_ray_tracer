use std::{error, fs, io, ops};

struct Color {
    red: u8,
    green: u8,
    blue: u8,
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
        for line in &self.lines {
            for pixel in line {
                writer.write(pixel.red.to_string().as_bytes())?;
                writer.write(b" ")?;
                writer.write(pixel.green.to_string().as_bytes())?;
                writer.write(b" ")?;
                writer.write(pixel.blue.to_string().as_bytes())?;
                writer.write(b"\n")?;
            }
        }
        Ok(())
    }

    fn sample() -> Image {
        let mut image = Image {
            width: 256,
            height: 256,
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

fn main() {
    println!("Hello, world!");

    let image = Image::sample();
    let mut file = fs::File::create("test.ppm").unwrap();
    image.draw(&mut file).unwrap();
}
