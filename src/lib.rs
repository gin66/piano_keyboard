//! # Piano_Keyboard
//!
//! This crates provides the graphical elements in order to draw a piano keyboard
//! with close to realistic appearance.
//!
//! As reference has been used internet resource displaying an
//! [octave drawing](http://www.rwgiangiulio.com/construction/manual/layout.jpg).
//! The dimensions described there have been used to create the elements of
//! a piano keyboard, which can be used to create for example an ovtave like this:
//! ![img](file:../../../keyboard.png)
//!
//! The graphical representation only provides the white and black areas for the keys,
//! and the keyboard rectangle, which is drawn gray in above picture.
//! It is visible, that between white keys and even between white and black keys a gap
//! is ensured.
//!
//! The gap between white and black keys can be remove by an option of the KeyboardBuilder.
//! 
//! The interface is prepared to be compatible an extention towards a 3d keyboard.
//! That's why the returned keyboard is called Keyboard2D and the related build function
//! is called build2d().
//!
/// This is just another rectangle definition.
///

mod base;
use crate::base::Base;

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
        blind: Option<Rectangle>
    },
    /// A black key consists of only one rectangle
    BlackKey(Rectangle),
    /// Finally the whole keyboard is returned as a rectangle, which is by one gap
    /// wider in each direction than the keyboard keys.
    Board(Rectangle),
}

/// The returned 2d Keyboard with all calculated elements.
pub struct Keyboard2d {
    pub left_white_key: u8,
    pub right_white_key: u8,
    pub width: u16,
    pub height: u16,
    pub unbalanced_a_g_keys: bool,
    elements: Vec<Element>
}
impl Keyboard2d {
    /// This function is the preferred way to iterate through all elements.
    /// The sequence is:
    ///
    /// * 1.: Keyboard 
    /// * 2.: Left outter white key
    /// * 3..n-1.: subsequent keys - white or black
    /// * n.: Right outter white key
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
                }=> {
                    rects.push(r1.clone());
                    rects.push(r2.clone());
                    if blind_as_white {
                        if let Some(blind) = opt_blind {
                            rects.push(blind.clone());
                        }
                    }
                },
                _ => ()
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
                },
                _ => ()
            }
        }
        rects
    }
}

/// The central builder to create a keyboard.
pub struct KeyboardBuilder {
    left_white_key: u8,
    right_white_key: u8,
    width: u16,
    dot_ratio_1024: u16, // dot height/dot width
    centered: bool,

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
        // 88 note piano range from A0 to C8
        KeyboardBuilder {
            left_white_key: 21,
            right_white_key: 108,
            width: 640,
            dot_ratio_1024: 1024,
            centered: true,

            // http://www.rwgiangiulio.com/construction/manual/layout.jpg
            // below measures are in 10 µm
            white_key_wide_width_10um: 22_15,
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
    pub fn is_rd64(mut self) -> KeyboardBuilder {
        // RD-64 is A1 to C7
        self.left_white_key = 21+12;
        self.right_white_key = 108-12;
        self
    }
    /// The keys are defined by MIDI key codes.
    /// This means the values have to be in range 0..128, with 0 representing C_-1.
    /// Thus a normal 88 keyboard, which is the default, uses key codes 21 (A_0) to 108 (C_8).
    pub fn set_most_left_right_white_keys(mut self,
                  left_white_key: u8, right_white_key: u8) -> Result<KeyboardBuilder,String> {
        if left_white_key > right_white_key{
            Err("left white key right from right white key ".to_string())
        }
        else if left_white_key > 127 {
            Err("left white key is out of range".to_string())
        }
        else if right_white_key > 127 {
            Err("right white key is out of range".to_string())
        }
        else if right_white_key - left_white_key < 11 {
            Err("Keyboard must be at least one octave".to_string())
        }
        else if !KeyboardBuilder::is_white(left_white_key) {
            Err("left white key is not a white key".to_string())
        }
        else if !KeyboardBuilder::is_white(right_white_key) {
            Err("right white key is not a white key".to_string())
        }
        else {
            self.left_white_key = left_white_key;
            self.right_white_key = right_white_key;
            Ok(self)
        }
    }
    /// Set the desired keyboard width in pixels. The built keyboard is regularly smaller
    /// than this value in order to have equal spacing - as perfect as possible.
    pub fn set_width(mut self, width: u16) -> Result<KeyboardBuilder,String> {
        if width > 65535-127 {
            Err("Requested width too big".to_string())
        }
        else {
            self.width = width;
            Ok(self)
        }
    }
    pub fn centered(mut self, centered: bool) -> KeyboardBuilder {
        self.centered = centered;
        self
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
    pub fn build2d(self) -> Keyboard2d {
        let base = Base::calculate(&self);

        let nr_of_white_keys = (self.left_white_key..=self.right_white_key)
                                .filter(|k| KeyboardBuilder::is_white(*k))
                                .count() as u16;

        let key_gap_10um = self.white_key_height_10um - self.black_key_height_10um
                                                  - self.white_key_wide_height_10um;

        // left and right from the outer keys have a gap, too
        let keyboard_width_10um = (self.white_key_wide_width_10um + key_gap_10um)
                                    * nr_of_white_keys as u32 + key_gap_10um;

        let key_gap = ((self.width as u32 * key_gap_10um + keyboard_width_10um/2)
                                                / keyboard_width_10um) as u16;
        let black_gap = if self.need_black_gap { 
            key_gap
        }
        else {
            0
        };

        let max_pure_white_key_width = self.width - key_gap * (nr_of_white_keys+1) as u16;

        let white_key_wide_width = max_pure_white_key_width / nr_of_white_keys;

        let real_width = (white_key_wide_width + key_gap) * nr_of_white_keys + key_gap;

        let center_offset = (self.width-real_width as u16)/2;

        let black_key_width = (((white_key_wide_width as u32) * self.black_key_width_10um
                                    + self.white_key_wide_width_10um/2)
                                    / self.white_key_wide_width_10um) as u16;

        let sum_white_key_small_width_cde = 
            3 * white_key_wide_width + 2 * key_gap - 2 * black_key_width
                - 4*black_gap;
        let (white_key_small_width_ce,white_key_small_width_d) = match sum_white_key_small_width_cde % 3 {
            0 => (sum_white_key_small_width_cde/3, sum_white_key_small_width_cde/3),
            1 => (sum_white_key_small_width_cde/3, sum_white_key_small_width_cde/3+1),
            2 => (sum_white_key_small_width_cde/3+1, sum_white_key_small_width_cde/3),
            _ => panic!("impossible") 
        };
        
        let sum_white_key_small_width_fbga = 
            4 * white_key_wide_width + 3 * key_gap
             - 3 * black_key_width - 6 * black_gap;

        let white_key_small_width_g = (sum_white_key_small_width_fbga as u32
                * self.white_key_small_width_ga_10um
                / (self.white_key_small_width_ga_10um+self.white_key_small_width_fb_10um)
                / 2) as u16;

        // if sum_white_key_small_width_fbga is uneven, then there is no good solution.
        // => enlarge a
        let white_key_small_width_fb = sum_white_key_small_width_fbga / 2 - white_key_small_width_g;
        let white_key_small_width_a = sum_white_key_small_width_fbga
                                        - 2 * white_key_small_width_fb
                                        -     white_key_small_width_g;

        let unbalanced_a_g_keys = white_key_small_width_a != white_key_small_width_g;

        let black_key_height = ((white_key_wide_width as u64 
                                    * self.black_key_height_10um as u64 * 1024
                                + self.white_key_wide_width_10um as u64 *1024/2)
                                / self.white_key_wide_width_10um as u64
                                / self.dot_ratio_1024 as u64) as u16;
        let white_key_wide_height = ((white_key_wide_width as u64
                                        * self.white_key_wide_height_10um as u64
                                + self.white_key_wide_width_10um as u64 * 1024/2)
                                / self.white_key_wide_width_10um as u64) as u16;

        let height = 2*key_gap + black_gap + black_key_height + white_key_wide_height;

        println!("#white={}",nr_of_white_keys);
        println!("width/gap={}/{}",white_key_wide_width,key_gap);
        println!("keyboard_width_10um={}",keyboard_width_10um);
        println!("real_width={}",real_width);
        println!("center_offset={}",center_offset);

        let mut elements = vec![];

        let board_rect = Rectangle {
            x: center_offset,
            y: 0,
            width: real_width,
            height: 50,
        };
        elements.push(Element::Board(board_rect));

        let mut small_offsets = vec![0];
        let off = white_key_small_width_ce + black_gap;
        small_offsets.push(off); // c#
        let off = off + black_key_width + black_gap - white_key_wide_width-key_gap;
        small_offsets.push(off); // d
        let off = off + white_key_small_width_d + black_gap;
        small_offsets.push(off); // d#
        let off = off + black_key_width + black_gap - white_key_wide_width-key_gap;
        small_offsets.push(off); // e

        small_offsets.push(0); // f
        let off = white_key_small_width_fb + black_gap;
        small_offsets.push(off); //f# 
        let off = off + black_key_width + black_gap - white_key_wide_width-key_gap;
        small_offsets.push(off); // g
        let off = off + white_key_small_width_g + black_gap;
        small_offsets.push(off); // g#
        let off = off + black_key_width + black_gap - white_key_wide_width-key_gap;
        small_offsets.push(off); // a
        let off = off + white_key_small_width_a + black_gap;
        small_offsets.push(off); // a#
        let off = off + black_key_width + black_gap - white_key_wide_width-key_gap;
        small_offsets.push(off); // b

        let mut white_x = key_gap;
        let mut next_white_x = key_gap;
        if self.centered {
            white_x += center_offset;
            next_white_x += center_offset;
        }
        for key in self.left_white_key..=self.right_white_key {
            if KeyboardBuilder::is_white(key) {
                white_x = next_white_x;
                let wide_rect = Rectangle {
                    x: white_x,
                    y: black_gap + black_key_height + key_gap,
                    width: white_key_wide_width,
                    height: white_key_wide_height,
                };
                next_white_x += white_key_wide_width + key_gap;

                let key_width = match key % 12 {
                    0 => white_key_small_width_ce,
                    2 => white_key_small_width_d,
                    4 => white_key_small_width_ce,
                    5 => white_key_small_width_fb,
                    7 => white_key_small_width_g,
                    9 => white_key_small_width_a,
                    11 => white_key_small_width_fb,
                    _ => panic!("impossible")
                };
                let small_rect = Rectangle {
                    x: white_x + small_offsets[(key % 12) as usize],
                    y: key_gap,
                    width: key_width,
                    height: black_gap + black_key_height,
                };

                let opt_blind = match key {
                    k if k == self.left_white_key => {
                        if small_rect.x > wide_rect.x {
                            Some(Rectangle {
                                x: wide_rect.x,
                                y: key_gap,
                                width: small_rect.x-wide_rect.x,
                                height: black_gap + black_key_height,
                            })
                        }
                        else {
                            None
                        }
                    },
                    k if k == self.right_white_key => {
                        if small_rect.x + small_rect.width < wide_rect.x + wide_rect.width {
                            Some(Rectangle {
                                x: small_rect.x,
                                y: key_gap,
                                width: wide_rect.x+wide_rect.width -small_rect.x,
                                height: black_gap + black_key_height,
                            })
                        }
                        else {
                            None
                        }
                    },
                    _ => None
                };

                elements.push(Element::WhiteKey {
                    wide: wide_rect,
                    small: small_rect,
                    blind: opt_blind,
                });
            }
            else {
                let rect = Rectangle {
                    x: white_x + small_offsets[(key % 12) as usize],
                    y: key_gap,
                    width: black_key_width,
                    height: black_key_height,
                };
                elements.push(Element::BlackKey(rect));
            }
        }

        println!("{:#?}", elements);

        Keyboard2d {
            left_white_key: self.left_white_key,
            right_white_key: self.right_white_key,
            width: self.width,
            height,
            unbalanced_a_g_keys,
            elements
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::KeyboardBuilder;

    #[test]
    fn test_max_width() {
        let _keyboard = KeyboardBuilder::new()
                        .set_most_left_right_white_keys(0,127).unwrap()
                        .set_width(65535-127).unwrap()
                        .build2d();
    }
    #[test]
    fn test_several_widths() {
        for width in 65000..65535-127 {
            let _keyboard = KeyboardBuilder::new()
                            .set_most_left_right_white_keys(0,127).unwrap()
                            .set_width(width).unwrap()
                            .build2d();
        }
    }
}

