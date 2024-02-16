
use image_to_ascii::add;
use owo_colors::OwoColorize;
use std::{fs::File, io::Write};

fn main() {

    println!("My number is {}!", 10.green());
    dbg!(10.green());

    let file = File::create("./output.txt").unwrap();

    write!()



}