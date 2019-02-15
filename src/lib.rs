
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub enum Element {
    WhiteKey(Rectangle, Rectangle),
    BlackKey(Rectangle),
    Board(Rectangle),
}

pub struct Keyboard {
    left_white_key: u8,
    right_white_key: u8,
    width: u16,
    height: u16,
    elements: Vec<Element>
}
impl Keyboard {
    pub fn white_keys(&self) -> Vec<Rectangle> {
        let mut rects = vec![];
        for opt_element in self.elements.iter() {
            match opt_element {
                Element::WhiteKey(r1,r2) => {
                    rects.push(r1.clone());
                    rects.push(r2.clone());
                },
                Element::BlackKey(r) => {
                },
                Element::Board(r) => {
                }
            }
        }
        rects
    }
    pub fn black_keys(&self) -> Vec<Rectangle> {
        let mut rects = vec![];
        for opt_element in self.elements.iter() {
            match opt_element {
                Element::WhiteKey(r1,r2) => {
                },
                Element::BlackKey(r) => {
                    rects.push(r.clone());
                },
                Element::Board(r) => {
                },
            }
        }
        rects
    }
}

pub struct KeyboardBuilder {
    left_white_key: u8,
    right_white_key: u8,
    width: u16,
    dot_ratio_1024: u16, // dot height/dot width

    white_key_wide_width_um: u32,
    white_key_small_width_cde_um: u32,
    white_key_small_width_fb_um: u32,
    white_key_small_width_ga_um: u32,
    key_gap_um: u32,

    black_key_width_um: u32,
    black_key_height_um: u32,
    need_black_gap: bool,

    white_key_height_um: u32,
    white_key_wide_height_um: u32,
}
impl KeyboardBuilder {
    pub fn new() -> KeyboardBuilder {
        // 88 note piano range from A0 to C8
        KeyboardBuilder {
            left_white_key: 21,
            right_white_key: 108,
            width: 640,
            dot_ratio_1024: 1024,

            // http://www.rwgiangiulio.com/construction/manual/layout.jpg
            // below measures are in µm
            white_key_wide_width_um: 22_150,
            white_key_small_width_cde_um: 13_970,
            white_key_small_width_fb_um: 12_830,
            white_key_small_width_ga_um: 13_080,

            black_key_width_um: 11_000,
            black_key_height_um: 80_000,
            need_black_gap: true,

            white_key_height_um: 126_270,
            white_key_wide_height_um: 45_000,

            key_gap_um: 1_270, // use 126,27-80-45 for white key gap
        }
    }
    pub fn is_rd64(mut self) -> KeyboardBuilder {
        // RD-64 is A1 to C7
        self.left_white_key = 21+12;
        self.right_white_key = 108-12;
        self
    }
    pub fn set_most_left_right_white_keys(mut self,
                  left_white_key: u8, right_white_key: u8) -> Option<KeyboardBuilder> {
        if !KeyboardBuilder::is_white(left_white_key) {
            None
        }
        else if !KeyboardBuilder::is_white(right_white_key) {
            None
        }
        else {
            self.left_white_key = left_white_key;
            self.right_white_key = right_white_key;
            Some(self)
        }
    }
    pub fn set_width(mut self, width: u16) -> KeyboardBuilder {
        self.width = width;
        self
    }
    pub fn is_white(key: u8) -> bool {
        match key % 12 {
            0 | 2 | 4 | 5 | 7 | 9 | 11 => true,
            1 | 3 | 6 | 8 | 10 => false,
            _ => panic!("wrong value"),
        }
    }
    pub fn build(self) -> Keyboard {
        let nr_of_white_keys = (self.left_white_key..=self.right_white_key)
                                .filter(|k| KeyboardBuilder::is_white(*k))
                                .count() as u16;

        // left and right from the outer keys have a gap, too
        let keyboard_width_um = (self.white_key_wide_width_um + self.key_gap_um)
                                    * nr_of_white_keys as u32 + self.key_gap_um;

        let key_gap = ((self.width as u32 * self.key_gap_um + keyboard_width_um/2)
                                                / keyboard_width_um) as u16;
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

        let black_key_width = (((white_key_wide_width as u32) * self.black_key_width_um
                                    + self.white_key_wide_width_um/2)
                                    / self.white_key_wide_width_um) as u16;

        let sum_white_key_small_width_cde = 
            (3 * white_key_wide_width + 2 * key_gap - 2 * black_key_width
                - 4*black_gap);
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
                * self.white_key_small_width_ga_um
                / (self.white_key_small_width_ga_um+self.white_key_small_width_fb_um)
                / 2) as u16;

        // if sum_white_key_small_width_fbga is uneven, then there is no good solution.
        // => enlarge a
        let white_key_small_width_fb = sum_white_key_small_width_fbga / 2 - white_key_small_width_g;
        let white_key_small_width_a = sum_white_key_small_width_fbga
                                        - 2 * white_key_small_width_fb
                                        -     white_key_small_width_g;

        let black_key_height = ((white_key_wide_width as u32 * self.black_key_height_um
                                + self.white_key_wide_width_um/2)
                                / self.white_key_wide_width_um) as u16;
        let white_key_wide_height = ((white_key_wide_width as u32 * self.white_key_wide_height_um
                                + self.white_key_wide_width_um/2)
                                / self.white_key_wide_width_um) as u16;

        let height = 2*key_gap + black_gap + black_key_height + white_key_wide_height;

        println!("#white={}",nr_of_white_keys);
        println!("width/gap={}/{}",white_key_wide_width,key_gap);
        println!("keyboard_width_um={}",keyboard_width_um);
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

        let mut white_x = center_offset+key_gap;
        let mut next_white_x = center_offset+key_gap;
        for key in self.left_white_key..=self.right_white_key {
            if KeyboardBuilder::is_white(key) {
                white_x = next_white_x;
                let wide_rect = Rectangle {
                    x: white_x,
                    y: black_gap + black_key_height,
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
                    y: 0,
                    width: key_width,
                    height: black_gap + black_key_height,
                };

                elements.push(Element::WhiteKey(wide_rect, small_rect));
            }
            else {
                let rect = Rectangle {
                    x: white_x + small_offsets[(key % 12) as usize],
                    y: 0,
                    width: black_key_width,
                    height: black_key_height,
                };
                elements.push(Element::BlackKey(rect));
            }
        }

        println!("{:#?}", elements);

        Keyboard {
            left_white_key: self.left_white_key,
            right_white_key: self.right_white_key,
            width: self.width,
            height,
            elements
        }
    }
}
