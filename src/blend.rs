
use photon_rs::{colour_spaces, effects, multiple, native, PhotonImage};
use crate::core::OUTPUT_FOLDER;
use anyhow::{anyhow, Result};
use crate::argparse::ArgParse;
use  std::path::Path;
pub struct BlendManager;
impl BlendManager{

    pub fn open_image(path: &str) -> Result<PhotonImage> {

        photon_rs::native::open_image(path)
            .map_err(|e| anyhow!("image {} open error: {}", path, e))
    }

    pub fn blend_image_available(image: &PhotonImage, image2: &PhotonImage) -> bool {

        if image.get_height() == image2.get_height() && image.get_width() ==image2.get_width(){
            true
        }else {
            false
        }
    }

    pub fn image_save(image: PhotonImage, blend_mode:&str) -> Result<String>{
        let output = OUTPUT_FOLDER.read().unwrap();
        let output = if let Some(output) = output.as_ref(){
            output
        }else{
            return Err(anyhow!("No output folder specified"));
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

    pub fn enhance<'a>(photon_image: &'a mut PhotonImage, options: &ArgParse) -> Result<&'a PhotonImage>{
        let gamma = options.gamma;
        let saturation = options.saturation;
        let contrast = options.contrast;
        let brightness = options.brightness;

        colour_spaces::gamma_correction(photon_image, gamma, gamma, gamma);
        if saturation >= 0.0{
            colour_spaces::saturate_hsv(photon_image, saturation);
        }else{
            colour_spaces::desaturate_hsv(photon_image, saturation.abs());
        }

        effects::adjust_contrast(photon_image, contrast);

        if brightness >= 0.0 {
            colour_spaces::lighten_hsv(photon_image, brightness);
        }else{
            colour_spaces::darken_hsv(photon_image, brightness.abs());
        }

        Ok(photon_image)
    }
    
    pub fn blend(options: &ArgParse) -> Result<()>{
        let mut photon_image = Self::open_image(&options.image)?;
        let photon_image_2 = Self::open_image(&options.image2)?;

        if !Self::blend_image_available(&photon_image, &photon_image_2){
            return Err(anyhow!("the size of blend images must be the same"));
        }

        let blend_mode = options.blend_mode.blend_name();
        multiple::blend(&mut photon_image, &photon_image_2, &blend_mode);

        let photon_image = Self::enhance(&mut photon_image, &options)?;
        let save_path = Self::image_save(photon_image.clone(), &blend_mode)?;
        println!("blend image save to {}", save_path);
        Ok(())
    }

}
