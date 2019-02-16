///! Base builder dealing only with white keys and key gaps between white keys.
///

use crate::KeyboardBuilder;

const KEY_C: u8 = 0;
const KEY_CIS: u8 = 1;
const KEY_D: u8 = 2;
const KEY_DIS: u8 = 3;
const KEY_E: u8 = 4;
const KEY_F: u8 = 5;
const KEY_FIS: u8 = 6;
const KEY_G: u8 = 7;
const KEY_GIS: u8 = 8;
const KEY_A: u8 = 9;
const KEY_AIS: u8 = 10;
const KEY_B: u8 = 11;

#[derive(Debug)]
pub enum ResultElement {
    Key(u16,u8),
    Gap(u16),
}

#[derive(Debug)]
enum Element {
    IdenticalWhite(u8),
    IdenticalGap,
    GapBC,
    GapEF,
    KeyD(u8),
    KeyCDE(u8),
    KeyFGAB(u8),
    OutterGap,
    EnlargedOutterLeftKey(u8),
    EnlargedOutterRightKey(u8),
}
  
#[derive(Default,Debug)]
pub struct Base {
    width: u16,
    nr_of_white_keys: u16,

    elements: Vec<Element>,
    key_gap_min: u16,
    kw_width_min: u16,

    identical_key: u16,
    identical_gap: u16,
    gap_bc: u16,
    gap_ef: u16,
    outter_gaps: u16,
    outter_left_key: u16,
    outter_right_key: u16,
    width_d: u16,
    width_cde: u16,
    width_fgab: u16,

    nr_of_full_octaves: u16,
    nr_of_c: u16,
    nr_of_d: u16,
    nr_of_e: u16,
    nr_of_cde: u16,
    nr_of_f: u16,
    nr_of_g: u16,
    nr_of_a: u16,
    nr_of_b: u16,
    nr_of_fgab: u16,
    nr_of_bc_gaps: u16,
    nr_of_ef_gaps: u16,

    outter_gaps_enlarged: bool,
    bc_gaps_enlarged: bool,
    ef_gaps_enlarged: bool,
    d_key_enlarged: bool,
    alternating_d_key_enlarged: bool,
    end_keys_enlarged: bool,
    cde_keys_enlarged: bool,
    fgab_keys_enlarged: bool,
}

impl Base {
    pub fn calculate(kb: &KeyboardBuilder) -> Base {
        let mut base = Base::default();
        base.width = kb.width;

        // Derive key gap measure from the given dimensions
        let key_gap_10um = kb.white_key_height_10um - kb.black_key_height_10um
                                                  - kb.white_key_wide_height_10um;

        base.nr_of_white_keys = (kb.left_white_key..=kb.right_white_key)
                                .filter(|k| KeyboardBuilder::is_white(*k))
                                .count() as u16;

        // Calculate the total keyboard width.
        // Left and right from the outer keys have a gap, too
        let keyboard_width_10um = (kb.white_key_wide_width_10um + key_gap_10um)
                                    * base.nr_of_white_keys as u32 + key_gap_10um;

        // Calculate the lower values for key gap and white key
        base.key_gap_min = (key_gap_10um * kb.width as u32 / keyboard_width_10um) as u16;
        base.kw_width_min = (kb.white_key_wide_width_10um * kb.width as u32 / keyboard_width_10um) as u16;

        // If the above remainders sum up to more than 1, then kw_width_min should be increased
        if base.nr_of_white_keys * (base.kw_width_min + 1) + (base.nr_of_white_keys + 1) * base.key_gap_min <= base.width {
            base.kw_width_min += 1;
        }

        // Calculate the minimum and maximum widths based on key_gap/kw_width and variations +0/1
        let min_width = base.nr_of_white_keys * base.kw_width_min + (base.nr_of_white_keys + 1) * base.key_gap_min;
        let max_width = base.nr_of_white_keys * (base.kw_width_min + 1) + (base.nr_of_white_keys + 1) * base.key_gap_min;


        // Ensure proper result
        assert!(min_width <= kb.width);
        assert!(max_width >= kb.width);

        // Fill the elements
        base.elements.push(Element::IdenticalGap);
        for key in kb.left_white_key..=kb.right_white_key {
            if KeyboardBuilder::is_white(key) {
                base.elements.push(Element::IdenticalWhite(key));
                base.elements.push(Element::IdenticalGap);
            }
        }
        base.identical_key = base.kw_width_min;
        base.identical_gap = base.key_gap_min;

        // Derive some further data
        let mut possible_full_octave = false;
        let mut possible_cde = false;
        let mut possible_fgab = false;
        let mut possible_bc_gap = false;
        let mut possible_ef_gap = false;
        for key in kb.left_white_key..=kb.right_white_key {
            match key % 12 {
                KEY_C => {
                    base.nr_of_c += 1;
                    possible_cde = true;
                    possible_full_octave = true;
                    if possible_bc_gap {
                        base.nr_of_bc_gaps += 1;
                    }
                }
                KEY_D => {
                    base.nr_of_d += 1;
                }
                KEY_E => {
                    base.nr_of_e += 1;
                    possible_ef_gap = true;
                    if possible_cde {
                        base.nr_of_cde += 1;
                    }
                }
                KEY_F => {
                    base.nr_of_f += 1;
                    possible_fgab = true;
                    if possible_ef_gap {
                        base.nr_of_ef_gaps += 1;
                    }
                }
                KEY_G => {
                    base.nr_of_g += 1;
                }
                KEY_A => {
                    base.nr_of_a += 1;
                }
                KEY_B => {
                    base.nr_of_b += 1;
                    possible_bc_gap = true;
                    if possible_fgab {
                        base.nr_of_fgab += 1;
                    }
                    if possible_full_octave {
                        base.nr_of_full_octaves += 1;
                    }
                }
                _ => ()
            }
        }

        base.find_solution();

        base
    }

    fn current_width(&self) -> (u16,u16) {
        // Accumulate width of all elements and return result
        let w = self.elements
            .iter()
            .map(|e| match e {
                Element::IdenticalWhite(_) => self.identical_key,
                Element::IdenticalGap => self.identical_gap,
                Element::GapBC => self.gap_bc,
                Element::GapEF => self.gap_ef,
                Element::KeyD(_) => self.width_d,
                Element::KeyCDE(_) => self.width_cde,
                Element::KeyFGAB(_) => self.width_fgab,
                Element::OutterGap => self.outter_gaps,
                Element::EnlargedOutterLeftKey(_) => self.outter_left_key,
                Element::EnlargedOutterRightKey(_) => self.outter_right_key,
            })
            .sum();
        if w > self.width {
            panic!("calculated width should not be bigger than given width");
        }
        (w,self.width - w)
    }

    fn find_solution(&mut self) {
        let mut last_delta = 0;
        loop {
            let (current,delta) = self.current_width();
            println!("{}/{}",delta,self.nr_of_white_keys);

            if delta == 0 {
                return; // solution already found
            }

            // Avoid endless loop
            if delta == last_delta {
                panic!("{:?}\nno solution. remaining delta {}",self,delta);
            }
            last_delta = delta;

            // If delta equals number of white_keys+1, then increase gap
            if delta == self.nr_of_white_keys+1 {
                self.identical_gap += 1;
                continue;
            }

            // If delta equals number of white_keys, then increase key width
            if delta == self.nr_of_white_keys {
                self.identical_key += 1;
                continue;
            }

            // If increasing the gap is multiple of cde or fgab groups + 0..4,
            // then increase gap
            if delta >= self.nr_of_white_keys+1 {
                let rem = delta - self.nr_of_white_keys - 1;
                if rem % self.nr_of_cde <= 4 || rem % self.nr_of_fgab <= 4 {
                    self.identical_gap += 1;
                    continue;
                } 
            }

            // If increasing the white keys leads to multiple of cde or fgab groups,
            // then increase white keys
            if delta >= self.nr_of_white_keys {
                let rem = delta - self.nr_of_white_keys;
                if rem % self.nr_of_cde <= 4 || rem % self.nr_of_fgab <= 4 {
                    self.identical_key += 1;
                    continue;
                } 
            }

            // Try to make use of enlarged keys FGAB
            if delta >= self.nr_of_f+self.nr_of_g+self.nr_of_a+self.nr_of_b && !self.fgab_keys_enlarged {
                self.fgab_keys_enlarged = true;
                for i in 1..self.elements.len()-1 {
                    let key = match self.elements[i] {
                        Element::IdenticalWhite(key) => {
                            let kc = key % 12;
                            if kc != KEY_F && kc != KEY_G && kc != KEY_A && kc != KEY_B {
                                continue;
                            }
                            key
                        }
                        _ => continue
                    };
                    self.width_fgab = self.identical_key + 1;
                    self.elements[i] = Element::KeyFGAB(key)
                }
                continue;
            }

            // Try to make use of enlarged keys CDE
            if delta >= self.nr_of_c+self.nr_of_d+self.nr_of_e && !self.cde_keys_enlarged {
                self.cde_keys_enlarged = true;
                for i in 1..self.elements.len()-1 {
                    let key = match self.elements[i] {
                        Element::IdenticalWhite(key) => {
                            let kc = key % 12;
                            if kc != KEY_C && kc != KEY_D && kc != KEY_E {
                                continue;
                            }
                            key
                        }
                        _ => continue
                    };
                    self.width_cde = self.identical_key + 1;
                    self.elements[i] = Element::KeyCDE(key)
                }
                continue;
            }

            // Try to make use of enlarged gap between b and c
            if delta >= self.nr_of_bc_gaps && !self.bc_gaps_enlarged {
                self.bc_gaps_enlarged = true;
                for i in 3..self.elements.len()-1 {
                    match self.elements[i] {
                        Element::IdenticalWhite(key) => {
                            if key % 12 != KEY_C {
                                continue;
                            }
                        }
                        _ => continue
                    }
                    self.gap_bc = self.identical_gap + 1;
                    self.elements[i-1] = Element::GapBC
                }
                continue;
            }

            // Try to make use of enlarged gap between e and f
            if delta >= self.nr_of_ef_gaps && !self.ef_gaps_enlarged {
                self.ef_gaps_enlarged = true;
                for i in 3..self.elements.len()-1 {
                    match self.elements[i] {
                        Element::IdenticalWhite(key)
                        | Element::KeyFGAB(key) => {
                            if key % 12 != KEY_F {
                                continue;
                            }
                        }
                        _ => continue
                    }
                    self.gap_ef = self.identical_gap + 1;
                    self.elements[i-1] = Element::GapEF
                }
                continue;
            }

            // Try to make use of enlarged key D
            if delta >= self.nr_of_d && !self.d_key_enlarged && !self.alternating_d_key_enlarged {
                self.d_key_enlarged = true;
                for i in 3..self.elements.len()-1 {
                    let key = match self.elements[i] {
                        Element::IdenticalWhite(key)
                        | Element::KeyCDE(key) => {
                            if key % 12 != KEY_D {
                                continue;
                            }
                            key
                        }
                        _ => continue
                    };
                    self.elements[i] = Element::KeyD(key)
                }
                if self.cde_keys_enlarged {
                    self.width_d = self.width_cde + 1;
                } else {
                    self.width_d = self.identical_key + 1;
                }
                continue;
            }

            // If delta is up to 4, then enlarge both sides gap
            if delta <= 4 && !self.outter_gaps_enlarged {
                // Just enlarge left right gap
                self.outter_gaps_enlarged = true;
                self.elements[0] = Element::OutterGap;
                if delta % 2 == 0 {
                    let n = self.elements.len();
                    self.elements[n-1] = Element::OutterGap;
                }
                self.outter_gaps = self.identical_gap + 1;
                continue;
            }

            // If delta is 2, then enlarge both sides end key
            if delta == 2 && !self.end_keys_enlarged {
                // Just enlarge left right gap
                self.end_keys_enlarged = true;
                self.elements[1] = match self.elements[1] {
                    Element::IdenticalWhite(key) => {
                        self.outter_left_key = self.identical_key + 1;
                        Element::EnlargedOutterLeftKey(key)
                    },
                    Element::KeyCDE(key) => {
                        self.outter_left_key = self.width_cde + 1;
                        Element::EnlargedOutterLeftKey(key)
                    },
                    Element::KeyFGAB(key) => {
                        self.outter_left_key = self.width_fgab + 1;
                        Element::EnlargedOutterLeftKey(key)
                    },
                    Element::EnlargedOutterLeftKey(key) => {
                        self.outter_left_key += 1;
                        Element::EnlargedOutterLeftKey(key)
                    },
                    ref el => panic!("Should not happen: {:?}",el)
                };
                let n = self.elements.len();
                self.elements[n-2] = match self.elements[n-2] {
                    Element::IdenticalWhite(key) => {
                        self.outter_right_key = self.identical_key + 1;
                        Element::EnlargedOutterRightKey(key)
                    },
                    Element::KeyCDE(key) => {
                        self.outter_right_key = self.width_cde + 1;
                        Element::EnlargedOutterRightKey(key)
                    },
                    Element::KeyFGAB(key) => {
                        self.outter_right_key = self.width_fgab + 1;
                        Element::EnlargedOutterRightKey(key)
                    },
                    Element::EnlargedOutterRightKey(key) => {
                        self.outter_right_key += 1;
                        Element::EnlargedOutterRightKey(key)
                    },
                    ref el => panic!("Should not happen: {:?}",el)
                };
                continue;
            }

            // Try to make use of alternating enlarged key D
            if delta >= self.nr_of_d/2 && !self.d_key_enlarged && !self.alternating_d_key_enlarged {
                self.alternating_d_key_enlarged = true;
                let mut enlarge = delta == self.nr_of_d/2;
                for i in 3..self.elements.len()-1 {
                    let key = match self.elements[i] {
                        Element::IdenticalWhite(key)
                        | Element::KeyCDE(key) => {
                            if key % 12 != KEY_D {
                                continue;
                            }
                            enlarge = !enlarge;
                            if !enlarge {
                                continue;
                            }
                            key
                        }
                        _ => continue
                    };
                    self.elements[i] = Element::KeyD(key)
                }
                if self.cde_keys_enlarged {
                    self.width_d = self.width_cde + 1;
                } else {
                    self.width_d = self.identical_key + 1;
                }
                continue;
            }

        }
    }
    pub fn result(&self) -> (bool,Vec<ResultElement>) {
        let result_elements = self.elements.iter()
            .map(|e| match e {
                Element::IdenticalWhite(key) => ResultElement::Key(self.identical_key,*key),
                Element::IdenticalGap => ResultElement::Gap(self.identical_gap),
                Element::GapBC => ResultElement::Gap(self.gap_bc),
                Element::GapEF => ResultElement::Gap(self.gap_ef),
                Element::KeyD(key) => ResultElement::Key(self.width_d,*key),
                Element::KeyCDE(key) => ResultElement::Key(self.width_cde,*key),
                Element::KeyFGAB(key) => ResultElement::Key(self.width_fgab,*key),
                Element::OutterGap => ResultElement::Gap(self.outter_gaps),
                Element::EnlargedOutterLeftKey(key) => ResultElement::Key(self.outter_left_key,*key),
                Element::EnlargedOutterRightKey(key) => ResultElement::Key(self.outter_right_key,*key),
            })
            .collect::<Vec<_>>();
        let perfect = !self.d_key_enlarged && !self.alternating_d_key_enlarged && !self.end_keys_enlarged
                                           && !self.cde_keys_enlarged && !self.fgab_keys_enlarged;
        (perfect,result_elements)
    }
}

