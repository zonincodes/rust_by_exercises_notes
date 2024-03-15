mod chapters;

use crate::chapters::chapter_1::chapter_1::{MinMax, Point2D};

fn main() {
    let minmax = MinMax::new(0, 14);

    println!("Compare Structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range: MinMax = MinMax::new(-300, 300);
    let small_range: MinMax = MinMax::new(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        big = big_range,
        small = small_range
    );

    let point = Point2D::new(4.5, 6.8);
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug {:?}", point);
    println!("Binary {:b}", point)
}
