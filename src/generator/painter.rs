use gif::{Encoder, Frame, Repeat};
use image::ExtendedColorType;
use image::{ImageBuffer, ImageEncoder, Rgb};
use std::fs::File;

pub struct Painter {
    disable: bool,
    steps: Vec<Vec<Vec<i32>>>,
}

fn create_image_from_values(values: &Vec<Vec<i32>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Determine the dimensions of the image
    let height = values.len();
    let width = values[0].len();

    //println!("Width: {}, Height: {}", width, height);

    // Create a new image buffer
    let mut image = ImageBuffer::new(width as u32 * 5, height as u32 * 5);

    // Iterate over the values and set the corresponding pixel color in the image
    for (y, row) in values.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Map the value to an RGB color
            let color = to_color(value);

            // Set the pixel color in the image
            for i in 0..5 {
                for j in 0..5 {
                    image.put_pixel((x as u32 * 5) + i, (y as u32 * 5) + j, color);
                }
            }
        }
    }

    image
}

fn to_color(value: i32) -> Rgb<u8> {
    let color = match value {
        //0 => Rgb([0, 0, 0]), // Black
        0 => Rgb([120, 120, 120]),
        1 => Rgb([0, 0, 0]),        // White
        2 => Rgb([128, 128, 128]),  // Blue
        3 => Rgb([0, 255, 0]),      // Green
        4 => Rgb([255, 0, 0]),      // Red
        5 => Rgb([255, 165, 0]),    // Orange
        6 => Rgb([128, 0, 128]),    // Purple
        7 => Rgb([255, 255, 0]),    // Yellow
        8 => Rgb([0, 255, 255]),    // Cyan
        9 => Rgb([255, 0, 255]),    // Magenta
        10 => Rgb([255, 192, 203]), // Pink
        11 => Rgb([0, 128, 0]),     // Dark Green
        _ => Rgb([(value * 3) as u8, (value * 12) as u8, (value * 12) as u8]), // Gray
    };
    color
}

fn create_image_from_values_u(values: &Vec<Vec<u32>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Determine the dimensions of the image
    let height = values.len();
    let width = values[0].len();

    //println!("Width: {}, Height: {}", width, height);

    // Create a new image buffer
    let mut image = ImageBuffer::new(width as u32 * 5, height as u32 * 5);

    // Iterate over the values and set the corresponding pixel color in the image
    for (y, row) in values.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Map the value to an RGB color
            let color = to_color(value as i32);

            // Set the pixel color in the image
            for i in 0..5 {
                for j in 0..5 {
                    image.put_pixel((x as u32 * 5) + i, (y as u32 * 5) + j, color);
                }
            }
        }
    }
    // println!(
    //     "Image created {}x{}x{}",
    //     image.width(),
    //     image.height(),
    //     image.pixels().len()
    // );

    image
}

fn create_gif(steps: &Vec<Vec<Vec<i32>>>, file_path: &str) -> Result<(), std::io::Error> {
    // Create a new GIF encoder
    let mut encoder = Encoder::new(
        File::create(file_path)?,
        steps[0][0].len() as u16,
        steps[0].len() as u16,
        &[],
    )
    .unwrap();
    // Set the repeat count to 0 (infinite loop)
    let _ = encoder.set_repeat(Repeat::Infinite);

    // Iterate over the steps and create frames
    for step in steps {
        let image = create_image_from_values(&step);
        let width = image.width() as u16;
        let height = image.height() as u16;
        let mut pixels = image.into_raw();
        let gif_frame = Frame::from_rgb_speed(
            width,
            height,
            &mut pixels, // Pass a mutable reference to the raw pixel data
            1,
        );
        // Add the frame to the GIF
        //for _ in 0..3 {
        encoder.write_frame(&gif_frame).unwrap();
        //}
    }

    Ok(())
}

pub fn save_image_to_file(
    image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    file_path: &str,
) -> Result<(), std::io::Error> {
    // Open a file for writing
    let file = File::create(file_path)?;

    // Create a PNG encoder
    let encoder = image::codecs::png::PngEncoder::new(file);

    // Encode the image and write it to the file
    let _ = encoder.write_image(
        &image,
        image.width(),
        image.height(),
        ExtendedColorType::Rgb8,
    );

    Ok(())
}

impl Painter {
    pub fn new() -> Painter {
        Painter {
            steps: Vec::new(),
            disable: true,
        }
    }

    pub fn enable(&mut self) {
        self.disable = false;
    }

    pub fn add_step(&mut self, step: Vec<Vec<i32>>) {
        if self.disable {
            return;
        }
        self.steps.push(step);
    }

    pub fn paint(&self) {
        if self.disable {
            return;
        }
        let _ = create_gif(&self.steps, "dungeon.gif");
    }

    pub fn paint_image(&self, map: &Vec<Vec<i32>>, name: &str) {
        let image = create_image_from_values(&map);
        let _ = save_image_to_file(&image, name);
    }

    pub fn paint_image_u(&self, map: &Vec<Vec<u32>>, name: &str) {
        let image = create_image_from_values_u(&map);
        let _ = save_image_to_file(&image, name);
    }
}
