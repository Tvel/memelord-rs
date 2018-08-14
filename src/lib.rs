extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate textwrap;

//use std::fs::File;
//use std::io::prelude::*;
use imageproc::drawing::draw_text_mut;
use image::{Rgba, open, JPEG, DynamicImage};
use rusttype::{FontCollection, Scale};
use textwrap::fill;

pub fn make_panzer(text : &str) -> Vec<u8>
{
    let panzer = Vec::from(include_bytes!("../DejaVuSans.ttf") as &[u8]);
    let mut image =  image::load_from_memory(&panzer[..])
        .expect(&format!("Could not load image at {:?}", "panzer.png"));

    let warp_text = fill(text, 25);

    let x = 25;
    let y = 380;

    draw_white_text(&mut image, x, y, &warp_text);
    image_to_vec(&mut image)
}

fn draw_white_text(image : &mut DynamicImage, start_x : u32, start_y : u32, text : &str ) -> ()
{
    let font = Vec::from(include_bytes!("../panzer.png") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let height = 20.0;
    let scale = Scale { x: height * 1.0, y: height };
    let line_space = (height + (height / 4.0)) as u32;

    let mut y = start_y;
    for line in text.lines() {
        draw_text_mut(image, Rgba([255u8, 255u8, 255u8, 255u8]), start_x, y, scale, &font, line);
        y = y + line_space;
    }
}

fn image_to_vec(img : &mut DynamicImage) -> Vec<u8>
{
    let mut buf = Vec::new();
    img.write_to(&mut buf, JPEG).unwrap();

    buf
}
