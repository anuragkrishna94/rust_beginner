mod chapters {
    pub mod chapter5;
    pub mod chapter6;
    pub mod chapter8;
    pub mod chapter9;
    pub mod chapter10;
}

use crate::chapters::chapter5::chapter5_main;
use crate::chapters::chapter6::chapter6_main;
use crate::chapters::chapter8::chapter8_main;
use crate::chapters::chapter9::chapter9_main;
use crate::chapters::chapter10::chapter10_main;

fn main() {
    println!("Hello, world!");
    print_chapter_header(5);
    chapter5_main();
    print_chapter_header(6);
    chapter6_main();
    print_chapter_header(8);
    chapter8_main();
    print_chapter_header(9);
    chapter9_main();
    print_chapter_header(10);
    chapter10_main();
}

fn print_chapter_header(chapter_num: u8) {
    println!("---------------------------START CHAPTER {}---------------------------", chapter_num);
}