use anyhow::Result;
use opencv:: {
    prelude::*,
    imgproc,
    imgcodecs,
    highgui,
    core
};

pub fn warp_perspective() -> Result<()> {
    let path = String::from("Resources/cards.jpg");
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR)?;

    let src = core::Point2f::from((529, 142), (771, 190), (405, 395), (674, 457));  // error
    let w = 250;
    let h = 350;
    let dst = core::Point2f::from((0.0, 0.0), (w, 0.0), (0.0, h), (w, h)); // error

    let mut img_warp = Mat::default();
    let matrix = imgproc::get_perspective_transform(&src, &dst, core::DECOMP_LU)?;
    imgproc::warp_perspective(
        &img,
        &mut img_warp,
        &matrix,
        core::Size::from((w, h)),
        imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::default()
    )?;

    highgui::imshow("Image", &img)?;
    highgui::imshow("Image Warp", &img_warp)?;
    highgui::wait_key(0)?;
    Ok(())
}