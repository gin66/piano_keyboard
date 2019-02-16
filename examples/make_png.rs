extern crate piano_keyboard;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use png;
use png::HasParameters;

use crate::piano_keyboard::KeyboardBuilder;

fn main() -> Result<(), Box<std::error::Error>> {
    let width = 800;

    let keyboard = KeyboardBuilder::new()
                        .set_width(width as u16)?
                        .set_most_left_right_white_keys(24,35)?
                        .build2d();

    let height = keyboard.height as u32;

    let path = Path::new(r"keyboard.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
     let mut writer = encoder.write_header().unwrap();

    let mut data = vec![0;(4*width*height) as usize];

    for x in 0..width {
        for y in 0..height {
            let i = ((y*width+x)*4) as usize;
            data[i  ] = 150;
            data[i+1] = 150;
            data[i+2] = 150;
            data[i+3] = 255;
        }
    }

    for (color,rectangles) in vec![
         (vec![255,255,255,255],keyboard.white_keys(true).into_iter()),
         (vec![0,0,0,255      ],keyboard.black_keys().into_iter()) ].into_iter() {
    
        for rect in rectangles.into_iter() {
            for x in rect.x..(rect.x+rect.width) {
                for y in rect.y..(rect.y+rect.height) {
                    let i = ((y as u32*width+x as u32)*4) as usize;
                    for (j,c) in color.iter().enumerate() {
                        data[i+j] = *c;
                    }
                }
            }
        }
    }

    writer.write_image_data(&data).unwrap();

    println!("Dimension: {}*{}",height,width);

    Ok(())    
}
