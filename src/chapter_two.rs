use anyhow::Result;
use opencv:: {
    prelude::*,
    highgui,
    imgproc,
    imgcodecs
};

pub fn func() -> Result<()> {
    let path = String::from("Resources/rust_wallpaper.jpg");
    let img = imgcodecs::imread(&path, 1)?;
    
    // Converting to Gray
    let mut img_gray = Mat::default();  // decalre a variable of Mat type to be used for conversion
    imgproc::cvt_color(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0)?;   

    // Converting to Gaussian Blur 
    let mut img_blur = Mat::default();
    imgproc::
    
    highgui::imshow("Image", &img)?;
    highgui::imshow("Image Gray", &img_gray)?;
    highgui::wait_key(0)?;
    Ok(())
}