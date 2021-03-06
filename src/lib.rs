extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate textwrap;

use imageproc::drawing::draw_text_mut;
use image::{Rgba, DynamicImage};
use rusttype::{Font, Scale};
use textwrap::fill;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn make_cardinal(text : &str) -> Vec<u8>
{
    let path = Path::new("images").join("cardinal.jpg");
    let mut f = File::open(path).unwrap();

    let mut cardinal = Vec::new();
    // read the whole file
    f.read_to_end(&mut cardinal).unwrap();

    let mut image =  image::load_from_memory(&cardinal[..])
        .expect(&format!("Could not load image at {:?}", "cardinal.jpg"));

    let warp_text = fill(text, 25);

    let x = 50;
    let y = 50;

    draw_rgb_text(&mut image, x, y, 58.0, &warp_text);
    image_to_vec(&mut image)
}

pub fn make_panzer(text : &str) -> Vec<u8>
{
    let path = Path::new("images").join("panzer.png");
    let mut f = File::open(path).unwrap();

    let mut panzer = Vec::new();
    // read the whole file
    f.read_to_end(&mut panzer).unwrap();

    let mut image =  image::load_from_memory(&panzer[..])
        .expect(&format!("Could not load image at {:?}", "panzer.png"));

    let warp_text = fill(text, 25);

    let x = 25;
    let y = 380;

    draw_white_text(&mut image, x, y, 36.0, &warp_text);
    image_to_vec(&mut image)
}

fn draw_white_text(image : &mut DynamicImage, start_x : u32, start_y : u32, line_height : f32, text : &str ) -> ()
{
    let font_data: &[u8] = include_bytes!("../DejaVuSans.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let scale = rusttype::Scale { x: line_height * 1.0, y: line_height };
    let line_space = (line_height + (line_height / 4.0)) as u32;

    let mut y = start_y;
    for line in text.lines() {
        draw_text_mut(image, Rgba([255u8, 255u8, 255u8, 255u8]), start_x, y, scale, &font, line);
        y = y + line_space;
    }
}

fn draw_rgb_text(image : &mut DynamicImage, start_x : u32, start_y : u32, line_height : f32, text : &str ) -> ()
{
    let font_data: &[u8] = include_bytes!("../DejaVuSans.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let scale = Scale { x: line_height * 1.0, y: line_height };
    let line_space = (line_height + (line_height / 4.0)) as u32;

    let red = Rgba([255u8, 0u8, 0u8, 255u8]);
    let green = Rgba([0u8, 255u8, 0u8, 255u8]);
    let blue = Rgba([0u8, 0u8, 255u8, 255u8]);

    let mut y = start_y;
    let mut count = 1;
    for line in text.lines() {
        let rgb_val = match count {
            1 | 4 | 7 => red,
            2 | 5 | 8 => green,
            3 | 6 | 9 => blue,
            _ => Rgba([255u8, 255u8, 255u8, 255u8])
        };
        draw_text_mut(image, rgb_val, start_x, y, scale, &font, line);
        y = y + line_space;
        count = count + 1;
    }
}

fn image_to_vec(img : &mut DynamicImage) -> Vec<u8>
{
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageFormat::Jpeg).unwrap();

    buf
}
