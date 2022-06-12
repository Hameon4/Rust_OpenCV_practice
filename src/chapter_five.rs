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

    // (529, 142), (771.0, 190.0), (405.0, 395.0), (674.0, 457.0)
    // let arr1: [[f32; 2]; 4] = [[529.0, 142.0], [771.0, 190.0], [405.0, 395.0], [674.0, 457.0]];
    let v_src = vec![
        core::Point2f::from((529.0, 142.0)),
        core::Point2f::from((771.0, 190.0)),
        core::Point2f::from((405.0, 395.0)),
        core::Point2f::from((674.0, 457.0)),
    ];
    let src = types::VectorOfPoint2f::from(v_src);
    let w = 250.0;
    let h = 350.0;
   // let dst = core::Point2f::from((0.0, 0.0), (w, 0.0), (0.0, h), (w, h)); // error
    //let arr2: [[f32; 2]; 4] = [[0.0, 0.0], [w, 0.0], [0.0, h], [w, h]];
    let v_dst = vec![
        core::Point2f::from((0.0, 0.0)),
        core::Point2f::from((w, 0.0)),
        core::Point2f::from((0.0, h)),
        core::Point2f::from((w, h)),
    ];
    let dst = types::VectorOfPoint2f::from(v_dst);

    let mut img_warp = Mat::default();
    let matrix = imgproc::get_perspective_transform(&src, &dst, core::DECOMP_LU)?;
    imgproc::warp_perspective(
        &img,
        &mut img_warp,
        &matrix,
        core::Size::from((250, 350)),
        imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::default()
    )?;

    highgui::imshow("Image", &img)?;
    highgui::imshow("Image Warp", &img_warp)?;
    highgui::wait_key(0)?;
    Ok(())
}