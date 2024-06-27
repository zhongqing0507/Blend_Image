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
    Exclusion,
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
            Self::Exclusion => "exclusion".to_string(),
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
    if value < -1.0 || value > 1.0 {
        Err(format!("`{}` is out of range. It should be between -1.0 and 1.0", s))
    } else {
        Ok(value)
    }
}


fn contrast_value_parser(s: &str) -> Result<f32, String> {
    let value: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if value < -255.0 || value > 255.0 {
        Err(format!("`{}` is out of range. It should be between -255.0 and 255.0", s))
    } else {
        Ok(value)
    }
}

fn brightness_value_parser(s: &str) -> Result<f32, String>{
    let value: f32 = s.parse().map_err(|_| format!("`{}` is not a valid number", s))?;
    if value < -1.0 || value > 1.0 {
        Err(format!("`{}` is out of range. It should be between -1.0 and 1.0", s))
    } else {
        Ok(value)
    }
}

pub trait ArgParseProcess {
    type Error;
    fn output_folder(&self) -> Result<String>;

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
}
