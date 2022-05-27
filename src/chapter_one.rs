use anyhow::Result;
use opencv:: {
    prelude::*,
    videoio,
    highgui,
    imgcodecs,
};

// -->  READ IMAGE FILE <--
#[allow(dead_code)]
pub fn read_image() -> Result<()> {
    let path = String::from("Resources/rust_wallpaper.jpg");
    let img = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR)?;
    highgui::imshow("Image", &img)?;
    highgui::wait_key(0)?;
    Ok(())
}

// -->  READ VIDEO FILE <--
#[allow(dead_code)]
pub fn read_video() -> Result<()> {
    let path = String::from("Resources/optimus.mp4");
    let mut cap = videoio::VideoCapture::from_file(&path, 0)?;
    let mut img = Mat::default();
    loop {
        cap.read(&mut img)?;
        highgui::imshow("Video", &img)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // 113 = 'q'
            break;
        }
    }
    Ok(())
}

// -->  READ WEBCAM <--
#[allow(dead_code)]
pub fn read_webcam() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("Video", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 {
            break;
        }
    }
    Ok(())
}
