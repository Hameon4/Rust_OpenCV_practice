use anyhow::Result;
use opencv:: {
    prelude::*,
    highgui,
    imgproc,
    imgcodecs
};

pub fn draw_shapes() -> Result<()> {
    let path = String::from("Resources/rust_wallpaper.jpg");
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR)?;
    
}