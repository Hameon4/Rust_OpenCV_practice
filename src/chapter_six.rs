use anyhow::Result;
use opencv::{
    imgcodecs,
    highgui
};

pub fn color_detection() -> Result <()> {
    let path = String::from("Resources/lambo.png");
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR)?;

    

    highgui::imshow("Image", &img)?;
    highgui::wait_key(0)?;
    Ok(())
}