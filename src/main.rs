mod chapters {
    pub mod chapter5;
    pub mod chapter6;
}

use crate::chapters::chapter5::chapter5_main;
use crate::chapters::chapter6::chapter6_main;

fn main() {
    println!("Hello, world!");
    chapter5_main();
    chapter6_main();
}
