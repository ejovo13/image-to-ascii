mod cli;

pub use cli::Args;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageResult, Pixel};
use owo_colors::OwoColorize;

fn pixel_to_color(pixel: image::Rgba<u8>) -> owo_colors::Rgb {
    let data = pixel.0;
    owo_colors::Rgb(data[0], data[1], data[2])
}

/// A set of characters (from Black to White) used to depict luminescence in ASCII art.
struct CharacterRamp {
    ramp: Vec<char>,
    /// Scalar used to compute the index of which character to use
    luma_ratio: f64,
}

impl CharacterRamp {
    fn new(character_ramp: &str) -> Self {
        // Decompose into individual characters
        let ramp: Vec<char> = character_ramp.chars().collect();
        let luma_ratio = (ramp.len() - 1) as f64 / 255.0;
        CharacterRamp { ramp, luma_ratio }
    }

    fn get_char(&self, pixel: image::Rgba<u8>) -> char {
        let luma = pixel.to_luma();
        let index = (luma.0[0] as f64 * self.luma_ratio) as usize;
        self.ramp[index]
    }
}

/// Scale our image up and resize to a fixed width
fn scale_and_resize_img_fixed_width(
    img: DynamicImage,
    desired_width: u32,
    filter_type: FilterType,
) -> DynamicImage {
    let font_aspect_taller = 2.0;

    let dim = img.dimensions();
    let scaled_width = dim.0 as f64 * font_aspect_taller;
    let new_aspect_ration_h_to_w = dim.1 as f64 / scaled_width;

    let final_height = desired_width as f64 * new_aspect_ration_h_to_w;

    img.resize_exact(desired_width, final_height as u32, filter_type)
}

/// Read an image from a given path and apply routine transformations
fn read_image_to_print(
    img_path: &str,
    desired_width: u32,
    contrast: f32,
    filter_type: FilterType,
) -> ImageResult<DynamicImage> {
    let img = image::open(img_path)?;
    let img = scale_and_resize_img_fixed_width(img, desired_width, filter_type);
    ImageResult::Ok(img.adjust_contrast(contrast))
}

/// Print image to the screen.
///
/// By default, do not print any character whose alpha channel is 0.
///
/// # Arguments
///
/// * `img` - The image to display
/// * `pixel_callback` - A callback function that determines what string to print for each pixel
fn print_image<F>(img: image::DynamicImage, pixel_callback: F) -> ImageResult<()>
where
    F: Fn(image::Rgba<u8>) -> String,
{
    // Now iterate along the pixels
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if pixel.0[3] == 0 {
                print!(" ")
            } else {
                print!("{}", pixel_callback(pixel));
            }
        }
        println!();
    }

    ImageResult::Ok(())
}

pub fn print_image_with_ramp(
    img_path: &str,
    width: u32,
    contrast: f32,
    character_ramp: &str,
    filter_type: FilterType,
    drop: Option<String>,
) -> ImageResult<()> {
    let img = read_image_to_print(img_path, width, contrast, filter_type)?;
    let ramp = CharacterRamp::new(character_ramp);

    match drop {
        Some(s) => {
            let pixel_callback = |p: image::Rgba<u8>| {
                let color = pixel_to_color(p);
                let c = ramp.get_char(p);
                // Replace our character with a space if we need to drop it
                let c = if s.contains(c) { ' ' } else { c };

                format!("{}", c.color(color).bold())
            };
            print_image(img, pixel_callback)
        }
        None => {
            let pixel_callback = |p: image::Rgba<u8>| {
                let color = pixel_to_color(p);
                let c = ramp.get_char(p);
                format!("{}", c.color(color).bold())
            };
            print_image(img, pixel_callback)
        }
    }
}

/// Print image using the
pub fn print_image_pixelated(
    img_path: &str,
    width: u32,
    contrast: f32,
    filter_type: FilterType,
) -> ImageResult<()> {
    let img = read_image_to_print(img_path, width, contrast, filter_type)?;
    let c: char = '\u{2588}';
    let pixel_callback = |p: image::Rgba<u8>| {
        let color = pixel_to_color(p);
        format!("{}", c.color(color).bold())
    };

    print_image(img, pixel_callback)
}
