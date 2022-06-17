use anyhow::Result;
use opencv::{imgcodecs::*, highgui::*, imgproc::*, prelude::*, core::*};
use std::ptr;
use std::ptr::null_mut;

pub fn color_detection() -> Result <()> {
    let path = String::from("Resources/lambo.png");
    let img = imread(&path, IMREAD_COLOR)?;

    let mut img_hsv = Mat::default();
    let mut mask = Mat::default();

    let mut h_min = 0; let mut s_min = 110; let mut v_min = 153;
    let mut h_max = 19; let mut s_max = 240; let mut v_max = 255;

    cvt_color( &img, &mut img_hsv, COLOR_BGR2HSV, 0)?;
    named_window("Trackbars", WINDOW_AUTOSIZE)?;

    create_trackbar("Hue Min", "Trackbars", Some(&mut ptr::null()), 179, None)?;
    set_trackbar_pos("Hue Min", "Trackbars", h_min)?;

    create_trackbar("Hue Max", "Trackbars", Some(&mut ptr::null()), 179, None)?;
    set_trackbar_pos("Hue Max", "Trackbars", h_max)?;

    create_trackbar("Sat Min", "Trackbars", Some(&mut ptr::null()), 255, None)?;
    set_trackbar_pos("Sat Min", "Trackbars", s_min)?;

    create_trackbar("Sat Max", "Trackbars", Some(&mut ptr::null()), 255, None)?;
    set_trackbar_pos("Sat Max", "Trackbars", s_max)?;

    create_trackbar("Val Min", "Trackbars", Some(&mut ptr::null()), 255, None)?;
    set_trackbar_pos("Val Min", "Trackbars", v_min)?;

    create_trackbar("Val Max", "Trackbars", Some(&mut ptr::null()), 255, None)?;
    set_trackbar_pos("Val Max", "Trackbars", v_max)?;
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