use std::{error::Error, fs, io};

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
    fn draw(&self, writer: &mut (dyn io::Write + 'static)) -> Result<(), Box<dyn Error>> {
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

fn main() {
    println!("Hello, world!");

    let image = Image::sample();
    let mut file = fs::File::create("test.ppm").unwrap();
    image.draw(&mut file).unwrap();
}
