//mod chapter_one;
//mod chapter_two;
mod chapter_three;
mod chapter_four;
mod chapter_two;

fn main() {
    //chapter_one::read_image().expect("Some Error Happened");
    //chapter_one::read_video().expect("Some Error Happened");
    //chapter_one::read_webcam().expect("Some Error Happened");
    //chapter_two::basic_functions().expect("Some Error Happened");
    chapter_three::resize_and_crop().unwrap();
}