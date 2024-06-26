mod blend;
mod argparse;
mod core;
mod utils;
use clap::Parser;
use core::options_post_processing;
use blend::BlendManager;
use anyhow::Result;
use argparse::ArgParse;

fn main() -> Result<()>{
    let args = ArgParse::parse();
    options_post_processing(&args)?;
    BlendManager::blend(&args)?;
    println!("done");
    Ok(())
}
