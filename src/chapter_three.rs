use anyhow::Result;
use opencv:: {
    prelude::*,
    core,
    highgui,
    imgproc,
    imgcodecs
};

#[allow(dead_code)]
pub fn basic_functions() -> Result<()> {
    let path = String::from("Resources/rust_wallpaper.jpg");
    let img = imgcodecs::imread(&path, 1)?;

    //Resize
    let img_resize = Mat::default();
    imgproc::

    highgui::imshow("Image", &img)?;
    highgui::wait_key(0)?;
    Ok(())
}