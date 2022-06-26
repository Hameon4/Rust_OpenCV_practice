use anyhow::Result;
use opencv:: {prelude::*, core, highgui, imgproc,};

pub fn draw_shapes() -> Result<()> {
    // blank image
    let mut img = Mat::new_size_with_default(
        core::Size::from((512, 512)),
        core::CV_8UC3,
        core::Scalar::from((255.0, 255.0, 255.0)))?;

    // drawing a circle
    imgproc::circle(
        &mut img,
        core::Point::from((256, 256)),
        155,
        core::Scalar::from((0.0, 70.0, 255.0)),
        imgproc::FILLED,
        imgproc::LINE_8,
        0)?;

    // drawing a rectangle
    imgproc::rectangle(
        &mut img,
        core::Rect::from_points(core::Point::from((130, 226)),
                                    core::Point::from((382, 286))),
        core::Scalar::from((255.0, 255.0, 255.0)),
        imgproc::FILLED,
        imgproc::LINE_8,
        0)?;

    //drawing a line
    imgproc::line(
        &mut img,
        core::Point::from((130, 296)),
        core::Point::from((382, 296)),
        core::Scalar::from((255.0, 255.0, 255.0)),
        2,
        imgproc::LINE_8,
        0)?;

    // put text
    imgproc::put_text(
        &mut img, "OpenCV using Rust!",
        core::Point::from((137, 262)),
        imgproc::FONT_HERSHEY_DUPLEX,
        0.75,
        core::Scalar::from((0.0, 70.0, 255.0)),
        1,
        imgproc::LINE_8,
        false)?;

    highgui::imshow("Image", &img)?;
    highgui::wait_key(0)?;
    Ok(())
}