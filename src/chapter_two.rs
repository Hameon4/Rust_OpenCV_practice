use anyhow::Result;
use opencv:: {
    prelude::*,
    core,
    highgui,
    imgproc,
    imgcodecs
};

pub fn basic_functions() -> Result<()> {
    let path = String::from("Resources/rust_wallpaper.jpg");
    let img = imgcodecs::imread(&path, 1)?;

    // Converting to Gray
    let mut img_gray = Mat::default();  // decalre a variable of Mat type to be used for conversion
    imgproc::cvt_color(
        &img,
        &mut img_gray,
        imgproc::COLOR_BGR2GRAY,
        0)?;

    // Converting to Gaussian Blur
    let mut img_blur = Mat::default();
    imgproc::gaussian_blur(
        &img, &mut img_blur,
        core::Size::from((3, 3)),
        3.0,
        0.0,
        core::BORDER_DEFAULT)?;

    // Edge Detection (Canny Edge Detector - Most Popular method)
    // Usually when detecting edges, its source image is blurred
    let mut img_canny = Mat::default();
    imgproc::canny(
        &img_blur,
        &mut img_canny,
        150.0,
        250.0,
        3,
        false)?;

    // Dilation (increase thickness) and Erosion (decrease thickness)
    // last 4 parameters of both dilate and erode are defaults
    let mut img_dilation = Mat::default();
    let mut img_erosion = Mat::default();
    let kernel = imgproc::get_structuring_element(
        imgproc::MORPH_RECT,
        core::Size::from((5, 5)),
        core::Point::from((-1, -1)))?;
    imgproc::dilate(
        &img_canny,
        &mut img_dilation,
        &kernel,
        core::Point::from((-1, -1)),
        1,
        core::BORDER_CONSTANT,
        imgproc::morphology_default_border_value()?)?;

    imgproc::erode(
        &img_dilation,
        &mut img_erosion,
        &kernel,
        core::Point::from((-1, -1)),
        1,
        core::BORDER_CONSTANT,
        imgproc::morphology_default_border_value()?)?;

    highgui::imshow("Image", &img)?;
    highgui::imshow("Image Gray", &img_gray)?;
    highgui::imshow("Image Blur", &img_blur)?;
    highgui::imshow("Image Canny", &img_canny)?;
    highgui::imshow("Image Dilation", &img_dilation)?;
    highgui::imshow("Image Erosion", &img_erosion)?;
    highgui::wait_key(0)?;
    Ok(())
}