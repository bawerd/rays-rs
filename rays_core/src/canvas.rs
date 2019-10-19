use std::vec::Vec;
use std::cmp;

use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub size: u32,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: u32, height: u32, initial_color: Option<Color>) -> Canvas {
        Canvas {
            width,
            height,
            size: (width * height),
            pixels: vec![initial_color.unwrap_or(Color::new_black()); (width * height) as usize]
        }
    }


    fn pixel_pos(&self, x: u32, y: u32) -> u32 {
        (y * self.width) + cmp::min(x, self.width)  
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        let mut pos;

        pos = self.pixel_pos(x, self.height - y);
        if pos > self.size { pos = self.size-1 };

        self.pixels[pos as usize] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Option<Color> {
        let pos;

        pos = self.pixel_pos(x, y);
        if pos > self.size { return None }

        Some(self.pixels[pos as usize])
    }

   fn get_clamped_values(pixel: &Color, min: f64, max:f64) -> (u8, u8, u8) {
        (clamp(pixel.r * 256., min, max) as u8, clamp(pixel.g * 256., min, max) as u8, clamp(pixel.b * 256., min, max) as u8)
    } 

    pub fn to_ppm(&self) -> String {
        const ROW_BREAK: usize = 70;
        
        let header = format!("P3\n{} {}\n255\n", self.width, self.height);

        let mut body = String::with_capacity(self.pixels.len() * 4 + header.len());
        let mut row_length = 0;

        for (i, pixel) in self.pixels.iter().enumerate() {
            let (r,g,b) = Self::get_clamped_values(pixel, 0.0, 255.0);
            let triad = &format!("{} {} {} ", r, g, b)[..];
            
            row_length += triad.len();

            if row_length % ROW_BREAK < row_length {
                body.truncate(body.len()-1); 
                body.push('\n');
                body.push_str(triad);
                row_length = 0;
            } else {
                body.push_str(triad);
            }

            // End with newline
            if i == self.pixels.len()-1 { 
                body.truncate(body.len()-1); 
                body.push('\n');
            }
        }        

        header + &body 
    }
}

fn clamp<T: PartialOrd>(input: T, min: T, max:T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20, None);
        let black = Color::new_black();

        assert!(c.pixels.iter().any(|&color| color == black));
    }
    
    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20, None);
        let red = Color::new(1.0, 0.0, 0.0);

        // Handle overflow silently when calling write_pixel 
        // and return None if out of bounds when calling pixel_at
        c.write_pixel(10, 20, red);
        assert_eq!(c.pixel_at(10, 20), None);

        c.write_pixel(0, 19, red);
        assert_eq!(c.pixel_at(0, 19), Some(red));
    }

    #[test]
    fn constructing_ppm_header() {
        let c = Canvas::new(5, 3, None);

        let ppm = c.to_ppm();

        let ppm_test = "P3\n5 3\n255\n";

        assert_eq!(ppm.find(ppm_test), Some(0));
    }

    #[test]
    fn constructing_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3, None);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();
        let ppm_test = r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#;

        assert_eq!(ppm, ppm_test);
    }
    
    #[test]
    fn splitting_long_lines_in_ppm() {
        let c = Canvas::new(10, 2, Some(Color::new(1.0, 0.8, 0.6)));
        let ppm = c.to_ppm();
        let ppm_test = r#"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153
"#;

        assert_eq!(ppm, ppm_test);
    }

}