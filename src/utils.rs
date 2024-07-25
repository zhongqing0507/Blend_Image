use std::{fs::create_dir_all, path::Path};

use anyhow::{anyhow, Result};

pub fn makedirs(path: &str) -> Result<()>{
    let p = Path::new(&path);

    if p.exists(){
        if p.is_file(){
            return Err(anyhow!("{} is a file not a directory", path));
        }
    }else {
        create_dir_all(path)?;  
    }
    Ok(())
} 