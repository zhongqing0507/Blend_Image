use std::sync::RwLock;

use anyhow::{anyhow, Result};
use crate::{argparse::{ArgParse, ArgParseProcess}, utils::makedirs};
use lazy_static::lazy_static;

lazy_static!(
    pub static ref OUTPUT_FOLDER:RwLock<Option<String>> = RwLock::new(None);
);


pub fn options_post_processing(options: &ArgParse) -> Result<()> {

    if options.image.is_empty() || options.image2.is_empty() {
        return Err(anyhow!("No input file specified"));
    }
    let output_folder = options.output_folder()?;
    println!("Output folder: {:?}", output_folder);

    makedirs(&output_folder)?;
    *OUTPUT_FOLDER.write().unwrap() = Some(output_folder);

    Ok(())
}

