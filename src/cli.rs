use clap::{Parser, ValueEnum};
use image::imageops::FilterType;
use serde::Serialize;

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ClapFilter {
    #[default]
    /// Linear interpolation (Triangle filter). Distorts the original colors in the image
    Linear,
    /// Choose the nearest pixel boundary; retains original colors.
    Nearest,
    /// Classic guassian kernel. Distorts the original colors in the image
    Gaussian,
}

impl ClapFilter {
    /// Retrieve the image processing filter associated with the CLI argument
    fn get_filter(&self) -> FilterType {
        match self {
            ClapFilter::Linear => FilterType::Triangle,
            ClapFilter::Nearest => FilterType::Nearest,
            ClapFilter::Gaussian => FilterType::Gaussian,
        }
    }
}

#[derive(Parser)]
#[command(author = "Evan Voyles")]
pub struct Args {
    /// Path to the image to print.
    img_path: String,
    /// The character ramp to use for luminosity of a single pixel.
    ///
    /// In the default character ramp, '$' is used for black pixels and ' ' for white.
    ///
    /// [default: $@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,"^`'. ]
    ///
    /// Ex: image_to_ascii ./image.png --ramp='* '
    #[arg(long)]
    ramp: Option<String>,
    /// The width in characters of the printed image.
    #[arg(long)]
    width: Option<u32>,
    /// Increase (or decrease) the contrast of our image. Units are in percent.
    ///
    /// [default: 50.0]
    ///
    /// Ex: image_to_ascii ./image.png --contrast=25
    #[arg(long)]
    contrast: Option<f32>,
    #[arg(short = 'p', long, default_value_t = false)]
    /// Print each pixel using \u2588, the "Full Block" Unicode character.
    pixelated: bool,
    /// Which kernel to use when computing the pixel values of resized images.
    #[arg(long, default_value_t, value_enum)]
    filter: ClapFilter,
    /// Drop any pixels whose grayscale character are contained in DROP.
    ///
    /// Ex: image_to_ascii ./image.png --drop="$"
    #[arg(long)]
    drop: Option<String>,
}

/* -------------------------------------------------------------------------- */
/*                             Default parameters                             */
/* -------------------------------------------------------------------------- */
const DEFAULT_RAMP: &str =
    r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,"^`'. "#;
const DEFAULT_CONTRAST: f32 = 50.0;
const DEFAULT_WIDTH: u32 = 80;

impl Args {
    pub fn get_ramp(&self) -> String {
        match &self.ramp {
            Some(ramp) => ramp.to_string(),
            None => DEFAULT_RAMP.to_string(),
        }
    }

    pub fn get_width(&self) -> u32 {
        match self.width {
            Some(w) => w,
            None => DEFAULT_WIDTH,
        }
    }

    pub fn get_contrast(&self) -> f32 {
        match self.contrast {
            Some(c) => c,
            None => DEFAULT_CONTRAST,
        }
    }

    pub fn get_filter(&self) -> FilterType {
        self.filter.get_filter()
    }

    pub fn is_pixelated(&self) -> bool {
        self.pixelated
    }

    pub fn get_img_path(&self) -> String {
        self.img_path.to_string()
    }

    pub fn get_drop(&self) -> Option<String> {
        self.drop.to_owned()
    }
}
