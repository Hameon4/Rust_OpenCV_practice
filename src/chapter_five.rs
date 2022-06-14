use anyhow::Result;
use opencv:: {
    prelude::*,
    imgproc,
    imgcodecs,
    highgui,
    core,
    types
};

pub fn warp_perspective() -> Result<()> {
    let path = String::from("Resources/cards.jpg");
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR)?;

    let v_src = vec![
        core::Point2f::from((748.0, 384.0)),
        core::Point2f::from((1021.0, 437.0)),
        core::Point2f::from((647.0, 709.0)),
        core::Point2f::from((965.0, 781.0)),
    ];

    let w = 250.0;
    let h = 350.0;

    let v_dst = vec![
        core::Point2f::from((0.0, 0.0)),
        core::Point2f::from((w, 0.0)),
        core::Point2f::from((0.0, h)),
        core::Point2f::from((w, h)),
    ];

    let src = types::VectorOfPoint2f::from(v_src);
    let dst = types::VectorOfPoint2f::from(v_dst);

    let mut img_warp = Mat::default();
    let matrix = imgproc::get_perspective_transform(&src, &dst, core::DECOMP_LU)?;
    imgproc::warp_perspective(
        &img,
        &mut img_warp,
        &matrix,
        core::Size::from((w as i32, h as i32)),
        imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::default()
    )?;

    highgui::imshow("Image", &img)?;
    highgui::imshow("Image Warp", &img_warp)?;
    highgui::wait_key(0)?;
    Ok(())
}