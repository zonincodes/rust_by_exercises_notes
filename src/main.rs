mod chapters;

use crate::chapters::{chapter_1::chapter_1::{MinMax, Point2D}, chapter_14::generics::{multiple_bounds::consume, the_problem}, chapter_8::closures::{self, closure_as_parameter}};
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
    println!("Binary {:b}", point);

    layered_option();

    closures::capturing();
    closure_as_parameter::closure_as_parameter();
    consume();
    the_problem::the_problem();

}
fn layered_option() {
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..(range + 1) {
        optional_integers.push(Some(i));
    }

    let mut cursor = range;

    // TODO: make this a while let statement - remember that vector.pop also
    // adds another layer of Option<T>. You can stack `Option<T>`s into
    // while let and if let.
    while let Some(integer) = optional_integers.pop() {
        if cursor == 0 {
            break;
        }
        assert_eq!(integer, Some(cursor));
        cursor -= 1;
    }

    assert_eq!(cursor, 0);
}