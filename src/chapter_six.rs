use anyhow::Result;
use opencv::{imgcodecs, highgui, imgproc, core, prelude::*, types};

pub fn color_detection() -> Result <()> {
    let path = String::from("Resources/lambo.png");
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR)?;

    let mut img_hsv = Mat::default();
    let mut mask = Mat::default();
    let hmin = 0; let smin = 110; let vmin = 153;
    let hmax = 19; let smax = 240; let vmax = 255;


    imgproc::cvt_color( &img, &mut img_hsv, imgproc::COLOR_BGR2HSV, 0)?;
    // inrange used to collect the color
    let v_lower = vec![core::Scalar::from((hmin))];
    let lower = types::VectorOfScalar::from(v_lower);
    let upper = core::Scalar::from((hmax, smax, vmax));
    core::in_range(&img_hsv, &lower, &upper, &mut mask)?;

    highgui::imshow("Image", &img)?;
    highgui::imshow("Image HSV", &img_hsv)?;
    highgui::imshow("Image Mask", &mask)?;
    highgui::wait_key(0)?;
    Ok(())
}