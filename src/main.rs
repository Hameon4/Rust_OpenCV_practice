//mod chapter_one;
//mod chapter_two;
mod chapter_three;

fn main() {
    //chapter_one::read_image().expect("Some Error Happened");
    //chapter_one::read_video().expect("Some Error Happened");
    //chapter_one::read_webcam().expect("Some Error Happened");
    //chapter_two::basic_functions().expect("Some Error Happened");
    chapter_three::main().unwrap();
}