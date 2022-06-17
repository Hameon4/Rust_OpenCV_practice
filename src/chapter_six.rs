use anyhow::Result;
use opencv::{imgcodecs::*, highgui::*, imgproc::*, prelude::*, core::*};
use opencv::highgui::TrackbarCallback;

pub fn color_detection() -> Result <()> {
    let path = String::from("Resources/lambo.png");
    let img = imread(&path, IMREAD_COLOR)?;

    let mut img_hsv = Mat::default();
    let mut mask = Mat::default();

    let mut h_min = 0; let mut s_min = 110; let mut v_min = 153;
    let mut h_max = 19; let mut s_max = 240; let mut v_max = 255;

    cvt_color( &img, &mut img_hsv, COLOR_BGR2HSV, 0)?;

    named_window("Trackbars", WINDOW_AUTOSIZE)?;
    create_trackbar("Hue Min", "Trackbars", Some(&mut h_min), 179, None)?;
    create_trackbar("Hue Max", "Trackbars", Some(&mut h_max), 179, None)?;
    create_trackbar("Sat Min", "Trackbars", Some(&mut s_min), 255, None)?;
    create_trackbar("Sat Max", "Trackbars", Some(&mut s_max), 255, None)?;
    create_trackbar("Val Min", "Trackbars", Some(&mut v_min), 255, None)?;
    create_trackbar("Val Max", "Trackbars", Some(&mut v_max), 255, None)?;
    loop {
        // inrange() used to collect the color
        let lower = Scalar::from((h_min as f64, s_min as f64, v_min as f64));
        let upper = Scalar::from((h_max as f64, s_max as f64, v_max as f64));
        in_range(&img_hsv, &lower, &upper, &mut mask)?;

        imshow("Image", &img)?;
        imshow("Image HSV", &img_hsv)?;
        imshow("Image Mask", &mask)?;
        wait_key(1)?;
    }
    Ok(())
}