mod blend;
mod argparse;
mod core;
mod utils;
mod blend_image;
mod adjuster;
use clap::Parser;
use core::options_post_processing;
use anyhow::Result;
use argparse::ArgParse;
use blend::BlendManager;
fn main() -> Result<()>{
    let args = ArgParse::parse();
    options_post_processing(&args)?;

    println!("args: {:?}", args);
    BlendManager::blend_manger(&args)?;
    println!("done");
    Ok(())
}
