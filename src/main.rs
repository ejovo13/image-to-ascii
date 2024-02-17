use clap::Parser;
use image::imageops::FilterType;
use im2as::{print_image_pixelated, print_image_with_ramp, Args};

fn main() {

    let args = Args::parse();
    let img_path = args.get_img_path();
    let width = args.get_width();
    let filter_type = args.get_filter();
    let ramp = args.get_ramp();
    let contrast = args.get_contrast();
    let drop = args.get_drop();

    if args.is_pixelated() {
        print_image_pixelated(&img_path, width, contrast, filter_type).unwrap();
    } else {
        print_image_with_ramp(&img_path, width, contrast, &ramp, filter_type, drop).unwrap();
    }
}