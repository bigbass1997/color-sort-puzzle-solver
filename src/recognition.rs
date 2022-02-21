use std::collections::vec_deque::VecDeque;
use image::{Rgb, RgbImage};
use palette::FromColor;
use crate::puzzle::TubeState;
use crate::PuzzleState;

#[derive(Clone, Debug, Default)]
pub struct ParsedTube {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub colors: VecDeque<u32>,
}
impl PartialEq for ParsedTube {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl ParsedTube {
    pub fn new(id: usize, x: u32, y: u32, width: u32, height: u32) -> Self { Self {
        id,
        x,
        y,
        width,
        height,
        colors: VecDeque::new(),
    }}
    
    pub fn inside(&self, x: u32, y: u32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}



pub fn parse_image(path: &str) -> PuzzleState {
    let img = image::open(path).unwrap().into_rgb8();
    let mut tubes: Vec<ParsedTube> = Vec::new();
    for y in 470..(img.height() - 170) {
        for x in 0..img.width() {
            let mut skip = false;
            for tube in &tubes {
                if tube.inside(x, y) {
                    skip = true;
                    break;
                }
            }
            if skip { continue }
            
            if pack(img.get_pixel(x, y)) == 0xBCBCBC {
                let mut tube = ParsedTube::new(tubes.len(), x - 11, y - 3, 134, 479);
                
                pop_colors(&mut tube, &img);
                
                tubes.push(tube);
            }
        }
    }
    
    let mut puzzle = PuzzleState::default();
    for tube in tubes {
        puzzle.0.push(TubeState::from(tube));
    }
    
    puzzle
}

fn pop_colors(tube: &mut ParsedTube, img: &RgbImage) {
    let x = tube.x + (tube.width / 2);
    
    let mut y = tube.y + (tube.height as f32 * 0.975) as u32;
    loop {
        let pixel = img.get_pixel(x, y);
        let color = pack(pixel);
        let hsv = palette::Hsv::from_color(palette::Srgb::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0));
        if hsv.value < 0.20 || (hsv.hue.to_positive_degrees() <= 0.0000001 && hsv.saturation <= 0.0000001) {
            break;
        }
        
        tube.colors.push_back(color);
        
        y -= 100;
    }
}

pub fn pack(pixel: &Rgb<u8>) -> u32 {
    ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32)
}