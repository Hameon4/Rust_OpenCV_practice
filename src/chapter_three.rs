use anyhow::Result;
use opencv:: {
    prelude::*,
    core,
    highgui,
    imgproc,
    imgcodecs
};

pub fn main() -> Result<()> {
    let path = String::from("Resources/rust_wallpaper.jpg");
    let img = imgcodecs::imread(&path, 1)?;

    // Resizing an image
    let mut img_resize = Mat::default();
    imgproc::resize(
        &img,
        &mut img_resize,
        core::Size::default(),
        0.5, 0.5,
        imgproc::INTER_LINEAR)?;
    //println!("Image Size: {:?}", img.size());
    //Image Size: Ok(Size_ { width: 728, height: 409 })

    // Cropping an image
    // roi ~ 'Region of Interest'
    let roi = core::Rect::new(200, 50, 300, 300);
    let img_crop = Mat::roi(&img, roi)?;


    highgui::imshow("Image", &img)?;
    highgui::imshow("Resized Image", &img_resize)?;
    highgui::imshow("Cropped Image", &img_crop)?;
    highgui::wait_key(0)?;
    Ok(())
}