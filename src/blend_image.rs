use std::path::Path;
use ndarray::Array1;
use anyhow::{anyhow, Result};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel};
use palette::{blend::{Blend, Compose}, LinSrgba};
use serde::{Deserialize, Serialize};
use crate::{adjuster::{BrightnessGammaContrastAdjuster, HueSaturationAdjuster}, argparse::ArgParseProcess};
use crate::{argparse::ArgParse, core::OUTPUT_FOLDER};

pub struct ImageIterator {
    width: u32,
    height: u32,
    item: u32,
}

impl ImageIterator {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            item: 0_u32,
        }
    }
    pub fn with_dimension(dimension: &(u32, u32)) -> Self {
        Self {
            width: dimension.0,
            height: dimension.1,
            item: 0_u32,
        }
    }
}

impl Iterator for ImageIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.item;
        self.item += 1;
        if n < (self.width * self.height) {
            Some((n / self.height, n % self.height))
        } else {
            None
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlendImage{
    raw_pixels:  Vec<u8>,
    width: u32,
    height: u32,
}

impl BlendImage{
    // pub fn new(raw_pixels: Vec<u8>, width: u32, height: u32) -> Self{
    //     Self{
    //         raw_pixels,
    //         width,
    //         height,
    //     }
    // }

    pub fn get_width(&self) -> u32{
        self.width
    }

    pub fn get_height(&self) -> u32{
        self.height
    }

    // pub fn get_raw_pixels(&self) ->&Vec<u8>{
    //     &self.raw_pixels
    // }
}


impl BlendImage{

    pub fn open_image(img_path: &str) -> Result<BlendImage>{
        let img = image::open(img_path)?;
        let (width, height) = img.dimensions();
        Ok(BlendImage{
            raw_pixels: img.to_rgba8().to_vec(),
            width,
            height, 
        })
    }

    pub fn save_image(img: BlendImage, img_path: &str) -> Result<()> {
        let raw_pixels = img.raw_pixels;
        let width = img.width;
        let height = img.height;
    
        let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
        let dynimage = DynamicImage::ImageRgba8(img_buffer);
    
        dynimage.save(img_path)?;
        Ok(())
    }
}

pub struct BlendManager;

impl BlendManager{

    fn dyn_image_from_raw(blend_image: &BlendImage) -> DynamicImage{
        let _len_vec = blend_image.raw_pixels.len() as u128;
        let raw_pixels = &blend_image.raw_pixels;
        let img_buffer = ImageBuffer::from_vec(
            blend_image.get_width(), 
            blend_image.get_height(), 
            raw_pixels.to_vec())
            .unwrap();

        DynamicImage::ImageRgba8(img_buffer) 
    }

    pub fn blend_manger(options: &ArgParse) -> Result<()>{
        let mut image = BlendImage::open_image(&options.image)?;
        let image2 = BlendImage::open_image(&options.image2)?;
        let blend_mode = options.blend_mode.blend_name();
        
        Self::enchance(&mut image, options)?;
        Self::blend(&mut image, &image2, &blend_mode)?;
        Self::image_save(image, &blend_mode)?;

        Ok(())
    }
    pub fn enchance(blend_image: &mut BlendImage, options: &ArgParse) -> Result<()>{
        let dyn_image = BlendManager::dyn_image_from_raw(blend_image);
        let mut image = dyn_image.to_rgba8();
        let bgc_adjuster = BrightnessGammaContrastAdjuster::new(
            options.brightness,
            options.contrast, 
            options.gamma);
        let colorize_color = options.parse_color()?;
        let hs_adjuster = HueSaturationAdjuster::new(
            options.saturation,
            options.colorize,
            &colorize_color,
            options.colorize_strength
        );


        for (_, _, pixel) in image.enumerate_pixels_mut(){
            bgc_adjuster.adjust_pixel(pixel);
            hs_adjuster.adjust_pixel(pixel);

        }

        let  blended_image = DynamicImage::ImageRgba8(image);
        blend_image.raw_pixels = blended_image.into_bytes();

        Ok(())
    }

    pub fn blend(blend_image: &mut BlendImage, blend_image2: &BlendImage, blend_mode: &str) -> Result<()>{
        let dyn_image = BlendManager::dyn_image_from_raw(blend_image);
        let dyn_image2 = BlendManager::dyn_image_from_raw(blend_image2);

        let (width, height) = dyn_image.dimensions();
        let (width2, height2) = dyn_image2.dimensions();
        if width != width2 || height != height2{
            return Err(anyhow!("the size of blend images must be the same"));
        }
        
        let mut image = dyn_image.to_rgba8();
        let image2 = dyn_image2.to_rgba8();

        for (x, y) in ImageIterator::new(width, height) {
            let pixel = image.get_pixel(x, y);
            let pixel2 = image2.get_pixel(x, y);
            
            let px_data = pixel.channels();
            let px_data2 = pixel2.channels();

            let components:(f32,f32, f32, f32);
            if blend_mode.to_lowercase().as_str() == "softlight" {
                let color = Array1::from_shape_fn(3, |i| px_data[i] as f32 / 255.0);
                let color_alpha = px_data[3] as f32 / 255.0;

                let color2 = Array1::from_shape_fn(3, |i| px_data2[i] as f32 / 255.0);
                let color2_alpha = px_data2[3] as f32 / 255.0;

                components = Self::softlight_op(color2, color, color2_alpha, color_alpha);
            }else{
                let color = LinSrgba::new(
                    px_data[0] as f32 / 255.0, 
                    px_data[1] as f32 / 255.0,
                    px_data[2] as f32 / 255.0, 
                    px_data[3] as f32 / 255.0, 
                ).into_linear();
    
    
                let color2  = LinSrgba::new(
                    px_data2[0] as f32 / 255.0, 
                    px_data2[1] as f32 / 255.0,
                    px_data2[2] as f32 / 255.0, 
                    px_data2[3] as f32 / 255.0, 
                ).into_linear();
                
                
                let blended = match blend_mode.to_lowercase().as_str() {
                    "overlay" => color.overlay(color2),
                    "over" => color2.over(color),
                    "atop" => color2.atop(color),
                    "xor" => color2.xor(color),
                    "addition" | "plus" => color2.plus(color),
                    "multiply" => color2.multiply(color),
                    "burn" => color.burn(color2),
                    "difference" => color2.difference(color),
                    "soft_light" | "soft light" | "softlight" => color2.soft_light(color),
                    "screen" => color2.screen(color),
                    "hard_light" | "hard light" | "hardlight" => color2.hard_light(color),
                    "dodge" => color.dodge(color2),
                    "subtract" | "exclusion" => color2.exclusion(color),
                    "lighten" => color2.lighten(color),
                    "darken" => color2.darken(color),
                    _ => color2.overlay(color),
                };
    
                components = blended.into_components();
            }
            

            image.put_pixel(
                x, 
                y, 
                image::Rgba([
                    (components.0 * 255.0) as u8, 
                    (components.1 * 255.0) as u8, 
                    (components.2 * 255.0) as u8, 
                    (components.3 * 255.0) as u8, 
                ])   
            );

        }
        
        let  blended_image = DynamicImage::ImageRgba8(image);
        blend_image.raw_pixels = blended_image.into_bytes();
        Ok(())
    }


    pub fn image_save(image: BlendImage, blend_mode:&str) -> Result<()>{
        let output = OUTPUT_FOLDER.read().unwrap();
        let output = if let Some(output) = output.as_ref(){
            output
        }else{
            return Err(anyhow!("No output folder specified"));
        };
        let filename = format!("blend_{}.png", blend_mode);
        let save_path = Path::new(&output).join(filename);
        let save_path = save_path.to_str().unwrap();

        BlendImage::save_image(image, save_path)?;
        Ok(())
        }

    fn softlight_op(dst: Array1<f32>, src: Array1<f32>, da: f32, sa: f32) -> (f32, f32, f32, f32) {
        let src2 = &src * 2.0;
        let dst_np = if da != 0.0 {
            (&dst * 1.0) / da
        } else {
            Array1::zeros(dst.len())
        };
    
        // 计算中间值的逐元素操作
        let temp = &src * (1.0 - da) + &dst * (1.0 - sa);
    
        let result: Vec<f32> = src2
            .iter()
            .zip(dst.iter())
            .zip(dst_np.iter())
            .zip(temp.iter())
            .map(|(((&src2, &dst), &dst_np), &temp)| {
                if src2 < sa {
                    (dst * (sa + (src2 - sa) * (1.0 - dst_np)) + temp) / 1.0
                } else if 4.0 * dst <= da {
                    (dst * sa + da * (src2 - sa)
                        * (((16.0 * dst_np - 12.0) * dst_np + 3.0) * dst_np / 1.0) + temp) / 1.0
                } else {
                    (dst * sa + da * (src2 - sa)
                        * ((dst_np.sqrt()) - dst_np) + temp) / 1.0
                }
            })
            .map(Self::clamp) // 确保每个通道值在0到1范围内
            .collect();
    
        (result[0], result[1], result[2], da)
    }
    
    fn clamp(value: f32) -> f32 {
        value.max(0.0).min(1.0)
    }
}
