use crate::KeyboardBuilder;
use crate::Base;
use crate::base::*;

#[derive(Debug)]
pub enum TopResultElement {
    WhiteGapBlack(u16,u16,u16),
    BlindWhiteGapBlack(u16,u16,u16,u16),
    BlindWhite(u16,u16),
}

#[derive(Default,Debug)]
pub struct Top {
    kb_width_min: u16,
    cde_pars: [u16;5],
    fgab_pars: [u16;7],
    cde_gap: u16,
    fgab_gap: u16,

    // calculated:
    cde_width: u16,
    fgab_width: u16,

    cde_key_width: u16,
    cde_black_key_width: u16,
    d_left_blind_width: u16,
    e_left_blind_width: u16,

    black_fs_as_width: u16,
    black_gs_width: u16,
    ga_white_width: u16,
    fb_white_width: u16,
    g_left_blind_width: u16,
    a_left_blind_width: u16,
    b_left_blind_width: u16,
}
impl Top {
    pub fn calculate(kb: &KeyboardBuilder, base: &Base) -> Top {
        let mut top = Top::default();

        top.kb_width_min = base.get_black_key_min_width();
        top.cde_pars = base.get_cde_pars();
        top.cde_width = top.cde_pars.iter().sum();
        top.fgab_pars = base.get_fgab_pars();
        top.fgab_width = top.fgab_pars.iter().sum();
        if kb.need_black_gap {
            top.cde_gap = base.get_cde_gap();
            top.fgab_gap = base.get_fgab_gap();
        }

        // cde-part
        // This contains two black keys and four gaps (optionally).
        // There can be two cases:
        //      cde_width is even => c,d,e white keys must be even
        //      cde_width is odd  => Thus c,d,e white keys must be odd
        //
        // In order to have same size white keys, multiple of three should be ensured.

        top.cde_black_key_width = match (top.cde_width - 2*top.kb_width_min - 4*top.cde_gap) % 3 {
            0 => top.kb_width_min,
            1 => top.kb_width_min + 2,
            2 => top.kb_width_min + 1,
            _ => panic!("cannot happen"),
        };
        top.cde_key_width = (top.cde_width - 2*top.cde_black_key_width - 4*top.cde_gap)/3;

        // fgab-part
        // This contains three black keys and six gaps (optionally).
        // There can be two cases:
        //      fgab_width is even => black_keys should be even or make middle key even
        //      fgab_width is odd  => black_keys should be odd or make middle key odd.

        top.black_fs_as_width = top.cde_black_key_width;
        top.black_gs_width = match (top.fgab_width % 2 == 0, top.cde_black_key_width % 2 == 0) {
            (true,true) => top.cde_black_key_width,
            (true,false) => top.cde_black_key_width+1,
            (false,true) => top.cde_black_key_width+1,
            (false,false) => top.cde_black_key_width,
        };
        let fgab_white_width = top.fgab_width - 2*top.black_fs_as_width - top.black_gs_width - 6 * top.fgab_gap;

        assert!(fgab_white_width % 2 == 0);

        // The distribution of width on the pairs g/a and f/b should be according to the um
        // In case fgab_width is not multiple of two, then f/b should be smaller than g/a
        let ga_white_width = ((fgab_white_width as u32 * kb.white_key_small_width_ga_10um as u32)
                                    / (kb.white_key_small_width_ga_10um + kb.white_key_small_width_fb_10um) as u32) as u16;
        let fb_white_width = ((fgab_white_width as u32 * kb.white_key_small_width_fb_10um as u32)
                                    / (kb.white_key_small_width_ga_10um + kb.white_key_small_width_fb_10um) as u32) as u16;
        let (ga_white_width, fb_white_width) = match (fgab_white_width - (ga_white_width + fb_white_width),fb_white_width % 2 == 0) {
            (0,true) => (ga_white_width,fb_white_width),
            (1,true) => (ga_white_width+1,fb_white_width),
            (2,true) => (ga_white_width+2,fb_white_width),
            (3,true) => (ga_white_width+1,fb_white_width+2),
            (0,false) => (ga_white_width+1,fb_white_width-1),
            (1,false) => (ga_white_width,fb_white_width+1),
            (2,false) => (ga_white_width+1,fb_white_width+1),
            (3,false) => (ga_white_width+2,fb_white_width+1),
            _ => panic!("Should not happen")
        };

        top.ga_white_width = ga_white_width;
        top.fb_white_width = fb_white_width;

        top.d_left_blind_width = top.cde_key_width + 2*top.cde_gap + top.cde_black_key_width - top.cde_pars[0..=1].iter().sum::<u16>();
        top.e_left_blind_width = 2*top.cde_key_width + 4*top.cde_gap + 2*top.cde_black_key_width - top.cde_pars[0..=3].iter().sum::<u16>();

        top.g_left_blind_width = top.fb_white_width/2 + 2*top.fgab_gap + top.black_fs_as_width - top.fgab_pars[0..=1].iter().sum::<u16>();
        top.a_left_blind_width = top.fb_white_width/2 + 4*top.fgab_gap + top.black_fs_as_width
                                + top.ga_white_width/2 + top.black_gs_width - top.fgab_pars[0..=3].iter().sum::<u16>();
        top.b_left_blind_width = top.fb_white_width/2 + 6*top.fgab_gap + 2*top.black_fs_as_width
                                + top.ga_white_width + top.black_gs_width - top.fgab_pars[0..=5].iter().sum::<u16>();

        top
    }
    pub fn is_perfect(&self) -> bool {
        self.black_fs_as_width == self.black_gs_width
    }
    pub fn get_top_for(&self, el: &ResultElement) -> TopResultElement {
        use crate::TopResultElement::*;
        match el {
            ResultElement::Key(width,key) => {
                // The correction is needed for alternating key d size
                let corr = match key % 12 {
                    KEY_C => width - self.cde_pars[0],
                    KEY_D => width - self.cde_pars[2],
                    KEY_E => width - self.cde_pars[4],
                    KEY_F => width - self.fgab_pars[0],
                    KEY_G => width - self.fgab_pars[2],
                    KEY_A => width - self.fgab_pars[4],
                    KEY_B => width - self.fgab_pars[6],
                    _ => 0
                };
                match key % 12 {
                    KEY_C => WhiteGapBlack(self.cde_key_width+corr,self.cde_gap,self.cde_black_key_width),
                    KEY_D => BlindWhiteGapBlack(self.d_left_blind_width,self.cde_key_width+corr,self.cde_gap,self.cde_black_key_width),
                    KEY_E => BlindWhite(self.e_left_blind_width,self.cde_key_width+corr),
                    KEY_F => WhiteGapBlack(self.fb_white_width/2+corr,self.fgab_gap,self.black_fs_as_width),
                    KEY_G => BlindWhiteGapBlack(self.g_left_blind_width,self.ga_white_width/2+corr,self.cde_gap,self.black_gs_width),
                    KEY_A => BlindWhiteGapBlack(self.a_left_blind_width,self.ga_white_width/2+corr,self.cde_gap,self.black_fs_as_width),
                    KEY_B => BlindWhite(self.b_left_blind_width,self.fb_white_width/2+corr),
                    _ => panic!("Should not happen")
                }
            },
            ResultElement::Gap(_) => panic!("Do not call with Gap")
        }
    }
}

