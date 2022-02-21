use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::rc::Rc;
use image::{ImageFormat, Rgb, RgbImage};
use palette::FromColor;
use crate::util::InfCell;

#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle {
    pub img: RgbImage,
    pub tubes: Vec<Tube>,
}
impl Puzzle {
    pub fn new(path: &str) -> Self {
        let mut path = PathBuf::from(path);
        let img = image::open(&path).unwrap().into_rgb8();
        let mut parsed = img.clone();
        
        let mut tubes: Vec<Tube> = Vec::new();
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
                
                let color = pack(parsed.get_pixel(x, y));
                
                if color == 0xBCBCBC {
                    let mut tube = Tube::new(tubes.len(), x - 11, y - 3, 134, 479);
                    
                    tube.pop_colors(&img);
                    
                    *parsed.get_pixel_mut(x, y) = [0xFF, 0x00, 0x00].into();
                    tube.draw_bounds(&mut parsed);
                    
                    println!("{:?}", tube);
                    
                    tubes.push(tube);
                }
            }
        }
        
        path.set_extension("parsed.png");
        parsed.save_with_format(path, ImageFormat::Png).unwrap();
        
        Self {
            img,
            tubes,
        }
    }
    
    pub fn solve(&self) {
        let permutation = InfCell::new(self.tubes.clone());
        
        for tube in permutation.get_mut() {
            if tube.colors.front().is_some() {
                for other in permutation.get_mut() {
                    if tube != other && other.colors.len() < 4 {
                        other.colors.push_front(tube.colors.pop_front().unwrap());
                    }
                }
            }
        }
        
        /*let mut queue = VecDeque::new();
        queue.push_back(initial);
        loop {
            
        }*/
    }
}

#[derive(Clone, Debug, Default)]
pub struct Tube {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub colors: VecDeque<u32>,
}
impl PartialEq for Tube {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl Tube {
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
    
    pub fn draw_bounds(&self, img: &mut RgbImage) {
        let color = Rgb([0xFF, 0x00, 0x00]);
        
        for x in self.x..(self.x + self.width) {
            *img.get_pixel_mut(x, self.y + self.height) = color;
            *img.get_pixel_mut(x, self.y) = color;
        }
        
        for y in self.y..(self.y + self.height) {
            *img.get_pixel_mut(self.x, y) = color;
            *img.get_pixel_mut(self.x + self.width, y) = color;
        }
    }
    
    pub fn pop_colors(&mut self, img: &RgbImage) {
        let x = self.x + (self.width / 2);
        
        let mut y = self.y + (self.height as f32 * 0.975) as u32;
        loop {
            let pixel = img.get_pixel(x, y);
            let color = pack(pixel);
            let hsv = palette::Hsv::from_color(palette::Srgb::new(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0));
            if hsv.value < 0.20 || (hsv.hue.to_positive_degrees() <= 0.0000001 && hsv.saturation <= 0.0000001) {
                break;
            }
            
            self.colors.push_front(color);
            println!("({}, {}) = {:#08X}", x, y, color);
            
            y -= 100;
        }
    }
}


fn pack(pixel: &Rgb<u8>) -> u32 {
    ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32)
}

pub fn solved(tubes: &Vec<Tube>) -> bool {
    for tube in tubes {
        if tube.colors.is_empty() { continue }
        let mut last = None;
        for color in &tube.colors {
            if last.is_none() {
                last = Some(*color);
            } else {
                if color.ne(last.as_ref().unwrap()) {
                    return false;
                }
            }
        }
    }
    
    true
}