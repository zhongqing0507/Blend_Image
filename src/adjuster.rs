use image::Rgba;
use palette::{FromColor, Hsl, Srgb};

#[derive(Debug)]
pub struct BrightnessGammaContrastAdjuster {
    brightness: f32,
    contrast: f32,
    gamma: f32,
    pub no_affect: bool,
}

impl BrightnessGammaContrastAdjuster {
    pub fn new(brightness: f32, contrast: f32, gamma: f32) -> Self {
        let contrast = ((contrast + 100.0) / 100.0).powi(2);
        let gamma = 1.0 / gamma;
        let no_affect = brightness == 0.0 && gamma == 1.0 && contrast == 0.0;
        BrightnessGammaContrastAdjuster { 
            brightness,
            contrast,
            gamma,
            no_affect, 
        }
    }

    fn clamp(value: f32, min_val: f32, max_val: f32) -> f32 {
        if value <= min_val {
            min_val
        } else if value >= max_val {
            max_val
        } else {
            value
        }
    }
    fn adjust_color_component(&self, c: u8) -> u8 {
        let v = (((c as f32 / 255.0 - 0.5) * self.contrast + 0.5) * 255.0 + self.brightness) / 255.0;
        let v = v.powf(self.gamma) * 255.0;
        // let a = Math.pow((((c / 255.0 - 0.5) * this.contrast_factor + 0.5) * 255.0 + this.brightness) / 255.0, this.gamma_correction) * 255.0;
        Self::clamp(v, 0.0, 255.0) as u8
    }

    pub fn adjust_pixel(&self, pixel: &mut Rgba<u8>) {
        if !self.no_affect {
            pixel[0] = self.adjust_color_component(pixel[0]);
            pixel[1] = self.adjust_color_component(pixel[1]);
            pixel[2] = self.adjust_color_component(pixel[2]);
        }
    }
}


#[derive(Debug)]
pub struct HueSaturationAdjuster {


    saturation: f32,
    no_affect: bool,
    colorize_on: bool,
    colorize_color:  (u8, u8, u8),
    colorize_h: f32,
    colorize_s: f32,
    colorize_strength: u8,

}


impl HueSaturationAdjuster {
    pub fn new(saturation: f32, colorize_on: bool, colorize_color: &Option<Vec<u8>>, colorize_strength: u8) -> Self {
        
        let saturation = saturation / 100.0 + 1.0;
        let no_affect = saturation == 0.0 && colorize_on == false;

        let (colorize_color, colorize_h, colorize_s) = 
            if let Some(color) = colorize_color {
            let rgb = Srgb::new(
                color[0] as f32 / 255.0, 
                color[1] as f32 / 255.0, 
                color[2] as f32 / 255.0);
            let hsl = Hsl::from_color(rgb);
            let h: f32 = hsl.hue.into();
            let s = hsl.saturation;
            ((color[0], color[1], color[2]), h , s)
        } else {
            ((255, 255, 255), 0.0 , 50.0)
        };

        Self {
            saturation,
            no_affect,
            colorize_on,
            colorize_color,
            colorize_h,
            colorize_s,
            colorize_strength,
        }
    }

    pub fn rgb_to_hsl(r: u8,g: u8,b: u8) -> (f32, f32, f32){
        let rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let hsl = Hsl::from_color(rgb);
        let (h,s,l) = hsl.into_components();
        let h = h.into();
        (h,s,l)

    }

    pub fn hsl_to_rgb(h: &mut f32, s: &mut f32, l: &mut f32) -> Srgb{
        let hsl = Hsl::new(*h, *s, *l);
        Srgb::from_color(hsl)
    }

    
    pub fn adjust_pixel_saturation(&self, pixel: &mut Rgba<u8>, h:&mut f32, s:&mut f32, l:&mut f32){
        if self.saturation != 1.0 {
            if self.saturation < 1.0 {
                *s = ((*s) * self.saturation).min(255.0);
            }else{
                *s = ((1.0 - (1.0 - *s / 255.0).powf(self.saturation.powf(2.0))) * 255.0)
                .min(255.0)
            }

            let rgb = Self::hsl_to_rgb(h,s,l);
            
            pixel[0] = (rgb.red * 255.0) as u8;
            pixel[1] = (rgb.green * 255.0) as u8;
            pixel[2] = (rgb.blue * 255.0) as u8;

        }
    }
    
    pub fn adjust_pixel_color(&self, pixel: &mut Rgba<u8>, h:&mut f32, s:&mut f32, l:&mut f32){
        if self.colorize_on{
            *h = self.colorize_h;
            *s = self.colorize_s;
            let colorized_color = Hsl::new(*h, *s, *l);
            let colorized_rgb = Srgb::from_color(colorized_color);
            if self.colorize_strength == 100{
                pixel[0] = (colorized_rgb.red * 255.0) as u8;
                pixel[1] = (colorized_rgb.green * 255.0) as u8;
                pixel[2] = (colorized_rgb.blue * 255.0) as u8;
            }else{
                let p = self.colorize_strength as f32 / 100.0;
                pixel[0] = (p * (colorized_rgb.red * 255.0) + (1.0 - p) * pixel[0] as f32) as u8;
                pixel[1] = (p * (colorized_rgb.green * 255.0) + (1.0 - p) * pixel[1] as f32) as u8;
                pixel[2] = (p * (colorized_rgb.blue * 255.0) + (1.0 - p) * pixel[2] as f32) as u8;    

                (*h, *s,*l) = Self::rgb_to_hsl(pixel[0], pixel[1], pixel[2]);    
            }
        }
    }
    pub fn adjust_pixel(&self, pixel: &mut Rgba<u8>){

        let (mut h, mut s, mut l) = Self::rgb_to_hsl(pixel[0], pixel[1], pixel[2]);

        self.adjust_pixel_saturation(pixel, &mut h, &mut s, &mut l);

        self.adjust_pixel_color(pixel, &mut h, &mut s, &mut l);
       
    }
}

