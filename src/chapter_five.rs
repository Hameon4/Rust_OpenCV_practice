use std::ops::Add;
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

    // (529, 142), (771.0, 190.0), (405.0, 395.0), (674.0, 457.0)
    // let arr1: [[f32; 2]; 4] = [[529.0, 142.0], [771.0, 190.0], [405.0, 395.0], [674.0, 457.0]];
    let p1 = core::Point2f::from((529.0, 142.0));
    let p2 = core::Point2f::from((771.0, 190.0));
    let p3 = core::Point2f::from((405.0, 395.0));
    let p4 = core::Point2f::from((674.0, 457.0));
    let src= core:
    let w = 250.0;
    let h = 350.0;
   // let dst = core::Point2f::from((0.0, 0.0), (w, 0.0), (0.0, h), (w, h)); // error
    //let arr2: [[f32; 2]; 4] = [[0.0, 0.0], [w, 0.0], [0.0, h], [w, h]];
    let q1 = core::Point2f::from((0.0, 0.0));
    let q2 = core::Point2f::from((w, 0.0));
    let q3 = core::Point2f::from((0.0, h));
    let q4 = core::Point2f::from((w, h));
    let dst = Mat::default();
    dst.add((q1, q2, q3, q4));

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