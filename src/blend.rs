
use photon_rs::{multiple::{self}, native, PhotonImage};
use crate::core::OUTPUT_FOLDER;
use anyhow::{anyhow, Result};
use crate::argparse::ArgParse;
use  std::path::Path;
pub struct BlendManager;
impl BlendManager{

    pub fn open_image(path: &str) -> Result<PhotonImage> {

        match photon_rs::native::open_image(path) {
            Ok(img) => {
                Ok(img)
            },

            Err(e) => {
                return Err(anyhow!("image: {} open error: {}", path, e));
            }
        }
    }

    pub fn blend_image_available(image: &PhotonImage, image2: &PhotonImage) -> bool {

        if image.get_height() == image2.get_height() && image.get_width() ==image2.get_width(){
            true
        }else {
            false
        }
    }

    pub fn image_save(image: PhotonImage, blend_mode:&str) -> Result<String>{
        let lock_output = OUTPUT_FOLDER.read().unwrap();
        let output = match lock_output.clone() {
            Some(output) => output,

            None => {
                return Err(anyhow!("No output folder specified"));
            }
        };

        let filename = format!("blend_{}.png", blend_mode);
        let save_path = Path::new(&output).join(filename);
        let save_path = save_path.to_str().unwrap();

        match native::save_image(image, save_path) {
            Ok(_) => {
                return Ok(save_path.to_string());
            },
            Err(e) => {
                return Err(anyhow!("Error saving image: {}", e));
            },
            }
        }


    pub fn blend(options: &ArgParse) -> Result<()>{
        let mut photon_image = Self::open_image(&options.image)?;
        let photon_image_2 = Self::open_image(&options.image2)?;

        if !Self::blend_image_available(&photon_image, &photon_image_2){
            return Err(anyhow!("the size of blend images must be the same"));
        }

        let blend_mode = options.blend_mode.blend_name();
        multiple::blend(&mut photon_image, &photon_image_2, &blend_mode);

        let save_path = Self::image_save(photon_image, &blend_mode)?;
        println!("blend image save to {}", save_path);
        Ok(())
    }

}
