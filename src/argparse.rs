use std::env;

use anyhow::{Result, anyhow};
use clap::{Parser, ValueEnum};
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
