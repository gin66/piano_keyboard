extern crate piano_keyboard;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use clap::value_t;
use clap::crate_version;
use clap::{App, Arg};
use png;
use png::HasParameters;

use crate::piano_keyboard::KeyboardBuilder;

pub fn usage() -> clap::ArgMatches<'static> {
    App::new("piano_keyboard demo")
        .version(crate_version!())
        .about("Example for piano_keyboard")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .default_value("800")
                .help("Set width of keyboard"),
        )
        .arg(
            Arg::with_name("left")
                .short("l")
                .long("left-white-key")
                .takes_value(true)
                .default_value("24")
                .help("Select left white key"),
        )
        .arg(
            Arg::with_name("right")
                .short("r")
                .long("right-white-key")
                .default_value("35")
                .takes_value(true)
                .help("Select right white key"),
        )
        .arg(
            Arg::with_name("no_gaps")
                .short("n")
                .long("no-gaps")
                .help("No gaps between black and white keys"),
        )
        .arg(
            Arg::with_name("A88")
                .long("a88")
                .help("Select 88 key Piano like Roland A88"),
        )
        .arg(
            Arg::with_name("RD64")
                .long("rd64")
                .help("Select 64 key Piano like Roland RD-64"),
        )
        .arg(Arg::with_name("verbose").multiple(true).short("v"))
        .arg(Arg::with_name("debug").short("d"))
        .get_matches()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = usage();

    let width = value_t!(matches, "width", u32).unwrap_or_else(|e| e.exit());

    let mut left_key = value_t!(matches, "left", u8).unwrap_or_else(|e| e.exit());
    let mut right_key = value_t!(matches, "right", u8).unwrap_or_else(|e| e.exit());

    if matches.is_present("RD64") {
        left_key = 21 + 12;
        right_key = 108 - 12;
    }
    if matches.is_present("A88") {
        left_key = 21;
        right_key = 108;
    }

    let keyboard = KeyboardBuilder::new()
        .set_width(width as u16)?
        .set_most_left_right_white_keys(left_key, right_key)?
        .white_black_gap_present(!matches.is_present("no_gaps"))
        .build2d();

    let height = keyboard.height as u32;

    let path = Path::new(r"keyboard.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = vec![0; (4 * width * height) as usize];

    for x in 0..width {
        for y in 0..height {
            let i = ((y * width + x) * 4) as usize;
            data[i] = 150;
            data[i + 1] = 150;
            data[i + 2] = 150;
            data[i + 3] = 255;
        }
    }

    for (color, rectangles) in vec![
        (
            vec![255, 255, 255, 255],
            keyboard.white_keys(true).into_iter(),
        ),
        (vec![0, 0, 0, 255], keyboard.black_keys().into_iter()),
    ]
    .into_iter()
    {
        for rect in rectangles.into_iter() {
            for x in rect.x..(rect.x + rect.width) {
                for y in rect.y..(rect.y + rect.height) {
                    let i = ((y as u32 * width + x as u32) * 4) as usize;
                    for (j, c) in color.iter().enumerate() {
                        data[i + j] = *c;
                    }
                }
            }
        }
    }

    writer.write_image_data(&data).unwrap();

    println!("Dimension: {}*{}", height, width);

    if keyboard.is_perfect() {
        println!("This is a perfect keyboard");
    } else {
        println!("This is not a perfect keyboard");
    }

    Ok(())
}
