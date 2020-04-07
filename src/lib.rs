//! # Piano_Keyboard
//!
//! [![Build Status](https://travis-ci.org/gin66/piano_keyboard.svg?branch=master)](https://travis-ci.org/gin66/piano_keyboard)
//!
//! This crate provides the graphical elements in order to draw a piano keyboard
//! with close to realistic, pixel accurate appearance.
//!
//! Reference for the dimension is this internet image:
//! ![octave drawing](http://www.rwgiangiulio.com/construction/manual/layout.jpg)
//! 
//! The dimensions described have been used to create the elements of
//! a piano keyboard like for an octave like this:
//! ![img](file:../../../keyboard.png)
//!
//! It is visible, that between white keys and even between white and black keys a gap
//! is ensured.
//!
//! The graphical representation only provides the white and black areas for the keys.
//! Those areas are represented by pixel accurate, non-overlapping rectangles.
//! No aliasing or similar is done on this level.
//!
//! Pixel accurate has the consequence, that in order to fill the requested width,
//! any gaps, white or black keys may need to be modified by up to one pixel.
//! Those changes may or may not be visible. If no adjustments have been made for
//! a given width and key range is reported by the function is_perfect()
//!
//! If the enlargement of various elements does not succeed, then as last resort
//! technique the outter gaps are enlarged.
//!
//! The gap between white and black keys can be removed by an option of the KeyboardBuilder.
//!
//! The interface is prepared to be compatible for an extension towards a 3d keyboard.
//! That's why the returned keyboard is called Keyboard2D and the related build function
//! is called build2d().

mod base;
mod top;
use crate::base::Base;
use crate::top::{Top, TopResultElement};

/// This is just another rectangle definition.
///
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// The elements provided by a Keyboard are white keys, black keys and the full keyboard - defined
/// by this enum.
#[derive(Debug)]
pub enum Element {
    /// A white key consists of up to three rectangles:
    ///     The wide part of the key.
    ///     The small part of the key next to tbe black keys.
    ///     For left/right outter keys, there may be a blind part for a non-existing black key.
    WhiteKey {
        wide: Rectangle,
        small: Rectangle,
        blind: Option<Rectangle>,
    },
    /// A black key consists of only one rectangle
    BlackKey(Rectangle),
}

/// The returned 2d Keyboard with all calculated elements.
pub struct Keyboard2d {
    pub left_white_key: u8,
    pub right_white_key: u8,
    pub width: u16,
    pub height: u16,
    perfect: bool,
    elements: Vec<Element>,
}
impl Keyboard2d {
    /// This function is the preferred way to iterate through all elements.
    /// The sequence is from left to right alternating keys in order:
    /// white,black,white,....,black,white
    ///
    pub fn iter(&self) -> std::slice::Iter<Element> {
        self.elements.iter()
    }
    /// This function allows to retrieve all white key rectangles - with or without blind.
    pub fn white_keys(&self, blind_as_white: bool) -> Vec<Rectangle> {
        let mut rects = vec![];
        for opt_element in self.elements.iter() {
            match opt_element {
                Element::WhiteKey {
                    wide: r1,
                    small: r2,
                    blind: opt_blind,
                } => {
                    rects.push(r1.clone());
                    rects.push(r2.clone());
                    if blind_as_white {
                        if let Some(blind) = opt_blind {
                            rects.push(blind.clone());
                        }
                    }
                }
                _ => (),
            }
        }
        rects
    }
    /// This function allows to retrieve all black key rectangles.
    pub fn black_keys(&self) -> Vec<Rectangle> {
        let mut rects = vec![];
        for opt_element in self.elements.iter() {
            match opt_element {
                Element::BlackKey(r) => {
                    rects.push(r.clone());
                }
                _ => (),
            }
        }
        rects
    }
    /// Is true for a keyboard without compromises on width/gaps.
    /// For example: cargo run --example make_png -- --width 811
    pub fn is_perfect(&self) -> bool {
        self.perfect
    }
}

/// The central builder to create a keyboard.
pub struct KeyboardBuilder {
    left_white_key: u8,
    right_white_key: u8,
    width: u16,
    dot_ratio_1024: u16, // dot height/dot width

    white_key_wide_width_10um: u32,
    //white_key_small_width_cde_10um: u32,
    white_key_small_width_fb_10um: u32,
    white_key_small_width_ga_10um: u32,

    black_key_width_10um: u32,
    black_key_height_10um: u32,
    need_black_gap: bool,

    white_key_height_10um: u32,
    white_key_wide_height_10um: u32,
}
impl KeyboardBuilder {
    pub fn new() -> KeyboardBuilder {
        KeyboardBuilder {
            left_white_key: 21,
            right_white_key: 108,
            width: 640,
            dot_ratio_1024: 1024,

            // http://www.rwgiangiulio.com/construction/manual/layout.jpg
            // below measures are in 10 µm
            white_key_wide_width_10um: 22_15,
            // following not needed, because assumption is equally spaced
            //white_key_small_width_cde_10um: 13_97,
            white_key_small_width_fb_10um: 12_83,
            white_key_small_width_ga_10um: 13_08,

            black_key_width_10um: 11_00,
            black_key_height_10um: 80_00,
            need_black_gap: true,

            white_key_height_10um: 126_27,
            white_key_wide_height_10um: 45_00,
        }
    }
    /// Define a standard piano with 25/37/49/61/64/73/76 or 88 keys.
    pub fn standard_piano(mut self, nr_of_keys: u8) -> Result<KeyboardBuilder, String> {
        let (left, right) = match nr_of_keys {
            88 => (21, 108),
            76 => (21, 108 - 12),          // one octave less from top
            73 => (21 + 3, 108 - 12),      // remove bottom a,a#,b
            64 => (21 + 12, 108 - 12),     // rd-64: covers full piano with one octave up/down
            61 => (21 + 12 + 3, 108 - 12), // remove bottom a,a#,b
            49 => (21 + 12 + 3, 108 - 24), // one octave less from top
            37 => (21 + 24 + 3, 108 - 24), // one octave less from bottom
            25 => (21 + 24 + 3, 108 - 36), // one octave less from top
            _ => {
                return Err(format!(
                    "size {} not a recognized standard size",
                    nr_of_keys
                ))
            }
        };
        assert_eq!(right - left + 1, nr_of_keys);
        self.left_white_key = left;
        self.right_white_key = right;
        Ok(self)
    }
    pub fn is_rd64(mut self) -> KeyboardBuilder {
        // RD-64 is A1 to C7
        self.left_white_key = 21 + 12;
        self.right_white_key = 108 - 12;
        self
    }
    /// The keys are defined by MIDI key codes.
    /// This means the values have to be in range 0..128, with 0 representing C_-1.
    /// Thus the default 88 keyboard uses key codes 21 (A_0) to 108 (C_8).
    pub fn set_most_left_right_white_keys(
        mut self,
        left_white_key: u8,
        right_white_key: u8,
    ) -> Result<KeyboardBuilder, String> {
        if left_white_key > right_white_key {
            Err("left white key right from right white key ".to_string())
        } else if left_white_key > 127 {
            Err("left white key is out of range".to_string())
        } else if right_white_key > 127 {
            Err("right white key is out of range".to_string())
        } else if right_white_key - left_white_key < 11 {
            Err("Keyboard must be at least one octave".to_string())
        } else if !KeyboardBuilder::is_white(left_white_key) {
            Err("left white key is not a white key".to_string())
        } else if !KeyboardBuilder::is_white(right_white_key) {
            Err("right white key is not a white key".to_string())
        } else {
            self.left_white_key = left_white_key;
            self.right_white_key = right_white_key;
            Ok(self)
        }
    }
    /// Sets the desired keyboard width in pixels.
    pub fn set_width(mut self, width: u16) -> Result<KeyboardBuilder, String> {
        if width > 65535 - 127 {
            Err("Requested width too big".to_string())
        } else {
            self.width = width;
            Ok(self)
        }
    }
    pub fn white_black_gap_present(mut self, gap_present: bool) -> KeyboardBuilder {
        self.need_black_gap = gap_present;
        self
    }
    fn is_white(key: u8) -> bool {
        match key % 12 {
            0 | 2 | 4 | 5 | 7 | 9 | 11 => true,
            1 | 3 | 6 | 8 | 10 => false,
            _ => panic!("wrong value"),
        }
    }
    /// Final build the keyboard, which means to perform all calculations and
    /// create all the elements.
    ///
    pub fn build2d(self) -> Keyboard2d {
        let base = Base::calculate(&self);
        let top = Top::calculate(&self, &base);

        let base_elements = base.get_elements();

        let nr_of_white_keys = (self.left_white_key..=self.right_white_key)
            .filter(|k| KeyboardBuilder::is_white(*k))
            .count() as u16;

        let key_gap_10um = self.white_key_height_10um
            - self.black_key_height_10um
            - self.white_key_wide_height_10um;

        // left and right from the outer keys have a gap, too
        let keyboard_width_10um = (self.white_key_wide_width_10um + key_gap_10um)
            * nr_of_white_keys as u32
            + key_gap_10um;

        let key_gap = ((self.width as u32 * key_gap_10um + keyboard_width_10um / 2)
            / keyboard_width_10um) as u16;
        let black_gap = if self.need_black_gap { key_gap } else { 0 };

        let max_pure_white_key_width = self.width - key_gap * (nr_of_white_keys + 1) as u16;

        let white_key_wide_width = max_pure_white_key_width / nr_of_white_keys;

        let black_key_height =
            ((white_key_wide_width as u64 * self.black_key_height_10um as u64 * 1024
                + self.white_key_wide_width_10um as u64 / 2)
                / self.white_key_wide_width_10um as u64
                / self.dot_ratio_1024 as u64) as u16;
        let white_key_wide_height = ((white_key_wide_width as u64
            * self.white_key_wide_height_10um as u64
            + self.white_key_wide_width_10um as u64 / 2)
            / self.white_key_wide_width_10um as u64) as u16;

        let height = 2 * key_gap + black_gap + black_key_height + white_key_wide_height;

        let mut elements = vec![];

        let mut white_x = 0;
        let n = base_elements.len() - 1;
        for (i, el) in base_elements.into_iter().enumerate() {
            match el {
                base::ResultElement::Key(width, _key) => {
                    let wide_rect = Rectangle {
                        x: white_x,
                        y: black_gap + black_key_height + key_gap,
                        width: width,
                        height: white_key_wide_height,
                    };
                    let tr = top.get_top_for(&el);
                    match tr {
                        TopResultElement::WhiteGapBlack(w, _g, _blk) => {
                            let small_rect = Rectangle {
                                x: white_x,
                                y: key_gap,
                                width: w,
                                height: black_gap + black_key_height,
                            };
                            let opt_blind = if i == n - 1 {
                                Some(Rectangle {
                                    x: white_x + w,
                                    y: key_gap,
                                    width: width - w,
                                    height: black_gap + black_key_height,
                                })
                            } else {
                                None
                            };
                            elements.push(Element::WhiteKey {
                                wide: wide_rect,
                                small: small_rect,
                                blind: opt_blind,
                            });
                        }
                        TopResultElement::BlindWhiteGapBlack(blind, w, g, _blk) => {
                            let opt_blind = if i == 1 {
                                Some(Rectangle {
                                    x: white_x,
                                    y: key_gap,
                                    width: blind,
                                    height: black_gap + black_key_height,
                                })
                            } else if i == n - 1 {
                                Some(Rectangle {
                                    x: white_x + w + g,
                                    y: key_gap,
                                    width: width - w - g,
                                    height: black_gap + black_key_height,
                                })
                            } else {
                                None
                            };
                            let small_rect = Rectangle {
                                x: white_x + blind,
                                y: key_gap,
                                width: w,
                                height: black_gap + black_key_height,
                            };
                            elements.push(Element::WhiteKey {
                                wide: wide_rect,
                                small: small_rect,
                                blind: opt_blind,
                            });
                        }
                        TopResultElement::BlindWhite(g, w) => {
                            let opt_blind = if i == 1 {
                                Some(Rectangle {
                                    x: white_x,
                                    y: key_gap,
                                    width: g,
                                    height: black_gap + black_key_height,
                                })
                            } else {
                                None
                            };
                            let small_rect = Rectangle {
                                x: white_x + g,
                                y: key_gap,
                                width: w,
                                height: black_gap + black_key_height,
                            };
                            elements.push(Element::WhiteKey {
                                wide: wide_rect,
                                small: small_rect,
                                blind: opt_blind,
                            });
                        }
                    };
                    if i < n - 1 {
                        match tr {
                            TopResultElement::WhiteGapBlack(w, g, blk) => {
                                let rect = Rectangle {
                                    x: white_x + w + g,
                                    y: key_gap,
                                    width: blk,
                                    height: black_key_height,
                                };
                                elements.push(Element::BlackKey(rect));
                            }
                            TopResultElement::BlindWhiteGapBlack(blind, w, g, blk) => {
                                let rect = Rectangle {
                                    x: white_x + blind + w + g,
                                    y: key_gap,
                                    width: blk,
                                    height: black_key_height,
                                };
                                elements.push(Element::BlackKey(rect));
                            }
                            TopResultElement::BlindWhite(_g, _w) => (),
                        }
                    };
                    white_x += width;
                }
                base::ResultElement::Gap(gap) => {
                    white_x += gap;
                }
            }
        }

        //println!("{:#?}", elements);

        Keyboard2d {
            left_white_key: self.left_white_key,
            right_white_key: self.right_white_key,
            width: self.width,
            height,
            perfect: base.is_perfect() && top.is_perfect(),
            elements,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::KeyboardBuilder;

    #[test]
    fn test_standard_pianos() -> Result<(), String> {
        let _keyboard = KeyboardBuilder::new()
            .standard_piano(88)?
            .standard_piano(76)?
            .standard_piano(73)?
            .standard_piano(64)?
            .standard_piano(61)?
            .standard_piano(49)?
            .standard_piano(37)?
            .standard_piano(25)?
            .set_width(800)
            .unwrap()
            .build2d();
        Ok(())
    }
    #[test]
    fn test_max_width() {
        let _keyboard = KeyboardBuilder::new()
            .set_most_left_right_white_keys(0, 127)
            .unwrap()
            .set_width(65535 - 127)
            .unwrap()
            .build2d();
    }
    #[test]
    fn test_several_widths() {
        for width in 65000..65535 - 127 {
            let _keyboard = KeyboardBuilder::new()
                .set_most_left_right_white_keys(0, 127)
                .unwrap()
                .set_width(width)
                .unwrap()
                .build2d();
        }
    }

    // Run this test with 
    //      cargo test -- --ignored
    #[test] #[ignore]
    fn test_all_pianos() -> Result<(), String> {
        for keys in vec![25,37,49,61,64,73,76,88].into_iter() {
            for width in 3*keys as u16..65535-127 {
                let _keyboard = KeyboardBuilder::new()
                    .standard_piano(keys)?
                    .set_width(width)
                    .unwrap()
                    .build2d();
            }
        }
        Ok(())
    }
}
