use std::env;

use anyhow::{Result, anyhow};
use clap::{Parser,ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum BlendMode {
    Overlay,
    Over,
    Atop,
    Xor,
    Multiply,
    Burn,
    Softlight,
    Hardlight,
    Difference,
    Lighten,
    Darken,
    Dodge,
    Plus,
    Addition,
    Exclusion,
    Subtract,
    Screen,
}

impl BlendMode{

    pub fn blend_name(&self) -> String{

        match self {
            Self::Overlay => "overlay".to_string(),
            Self::Over => "over".to_string(),
            Self::Atop => "atop".to_string(),
            Self::Xor => "xor".to_string(),
            Self::Multiply => "multiply".to_string(),
            Self::Burn => "burn".to_string(),
            Self::Softlight => "softlight".to_string(),
            Self::Hardlight => "hardlight".to_string(),
            Self::Difference => "difference".to_string(),
            Self::Lighten => "lighten".to_string(),
            Self::Darken => "darken".to_string(),
            Self::Dodge => "dodge".to_string(),
            Self::Plus => "plus".to_string(),
            Self::Addition => "addition".to_string(),
            Self::Exclusion => "exclusion".to_string(),
            Self::Subtract => "subtract".to_string(),
            Self::Screen => "screen".to_string(),
            
        }
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, PartialEq, Eq)]
pub enum Format {
    PNG,
    WEBP,
    JPEG,
    TIFF,
}

impl Format{
    pub fn format_name(&self) -> String{
        match self {
            Self::PNG => "png".to_string(),
            Self::WEBP => "webp".to_string(),
            Self::JPEG => "jpeg".to_string(),
            Self::TIFF => "tiff".to_string(),
        }
    }
}

fn gamma_value_parser(s: &str) -> Result<f32, String> {
    let value: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if value < 0.1 || value > 10.0 {
        Err(format!("`{}` is out of range. It should be between 0.1 and 10", s))
    } else {
        Ok(value)
    }
}

fn saturation_value_parser(s: &str) -> Result<f32, String> {
    let value: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if value < -100.0 || value > 100.0 {
        Err(format!("`{}` is out of range. It should be between -100.0 and 100.0", s))
    } else {
        Ok(value)
    }
}


fn contrast_value_parser(s: &str) -> Result<f32, String> {
    let value: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if value < -100.0 || value > 100.0 {
        Err(format!("`{}` is out of range. It should be between -100.0 and 100.0", s))
    } else {
        Ok(value)
    }
}

fn brightness_value_parser(s: &str) -> Result<f32, String>{
    let value: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if value < -255.0 || value > 255.0 {
        Err(format!("`{}` is out of range. It should be between -255.0 and 255.0", s))
    } else {
        Ok(value)
    }
}

fn colorize_strength_parse(s: &str) -> Result<u8, String>{
    let value: u8 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if  value > 100 {
        Err(format!("`{}` is out of range. It should be between -255.0 and 255.0", s))
    } else {
        Ok(value)
    }
}


pub trait ArgParseProcess {
    type Error;
    fn output_folder(&self) -> Result<String>;
    fn parse_color(&self) -> Result<Option<Vec<u8>>>;
}


#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct ArgParse{
    /// The path to the image, the basemap of blend image 
    pub image: String,

    /// The path to the image, The upper layer image of the blend
    pub image2: String,

    /// The blend image save path
    #[arg(short, long)]
    pub output: Option<String>,

    /// The blend mode, default is overlay
    #[arg(value_enum, short = 'm', long, default_value_t = BlendMode::Overlay)]
    pub blend_mode: BlendMode,

    #[arg(value_enum, long, default_value_t = Format::PNG)]
    pub format: Format,

    /// The gamma value, default is 1.0, range is [0.1, 10.0]
    #[arg(short, long, value_parser = gamma_value_parser, default_value_t = 1.0)]
    pub gamma: f32,

    /// The saturation value, default is 0.0 ,range is [-1.0, 1.0]
    #[arg(short, long, value_parser = saturation_value_parser, default_value_t = 0.0)]
    pub saturation: f32,

    /// The contrast value, default is 0.0, range is [-255.0, 255.0]
    #[arg(short, long, value_parser = contrast_value_parser, default_value_t = 0.0)]
    pub contrast: f32,

    /// The brightness value, default is 0.0, range is [-1.0, 1.0]
    #[arg(short, long, value_parser = brightness_value_parser, default_value_t = 0.0)]
    pub brightness: f32,

    #[arg(long, default_value_t = false)]
    pub colorize: bool,

    #[arg(long)]
    pub colorize_color: Option<String>,

    /// The colorize strength, default is 100, range is [-100, 100]
    #[arg(long, value_parser = colorize_strength_parse,  default_value_t = 100)]
    pub colorize_strength: u8,
}


impl ArgParseProcess for ArgParse{

    type Error = anyhow::Error;
    fn output_folder(&self) -> Result<String> {
        let output = match self.output {

            Some(ref output) => {
                if output.is_empty(){
                    return Err(anyhow!("Output folder is empty"));
                }

                output.to_string()
            },
            None => {
                let current_dir =  env::current_dir()?;
                current_dir.to_str().unwrap().to_string()
            }
        };
    
        Ok(output)
    }

    fn parse_color(&self) -> Result<Option<Vec<u8>>> {

        if let Some(colorize_color) = &self.colorize_color{
            if colorize_color.starts_with('#'){

                if colorize_color.len() != 7{
                    return Err(anyhow!("colorize color must be #RRGGBB"));
                } 
                // 解析十六进制颜色
                let parse_hex = |s: &str| -> Result<u8> {
                    u8::from_str_radix(s, 16).map_err(|_| anyhow!("Invalid hex value"))
                };
        
                let r = parse_hex(&colorize_color[1..3])?;
                let g = parse_hex(&colorize_color[3..5])?;
                let b = parse_hex(&colorize_color[5..7])?;
        
                Ok(Some(vec![r, g, b]))
            } else {
                // 解析逗号分隔的RGB值
                let str_parts: Vec<&str> = colorize_color.split(",").map(str::trim).collect();
                if str_parts.len() != 3 {
                    return Err(anyhow!("Invalid color format: {}. Expected 'R,G,B' or '#RRGGBB'", colorize_color));
                }
        
                let mut rgb = Vec::with_capacity(3);
                for part in str_parts {
                    match part.parse::<u8>() {
                        Ok(value) => rgb.push(value),
                        Err(_) => return Err(anyhow!("Invalid color component: {}", part)),
                    }
                }
        
                Ok(Some(rgb))
            }
        }else {
            Ok(None)
        }
        
    }
}