pub mod generics {
    mod traits {
        struct Empty;
        struct Null;

        // A trait generic over `T`
        trait DoubleDrop<T> {
            // Define a method on the caller type which takes
            // an additional single parameter `T` and doed nothing with it.
            fn double_drop(self, _: T);
        }

        impl<T, U> DoubleDrop<T> for U {
            // This method takes ownership of both passed arguments,
            //deallocating both
            fn double_drop(self, _: T) {}
        }
        #[allow(dead_code)]
        fn usage() {
            let empty = Empty;
            let null = Null;

            // Dealocate `empty` and `null`
            empty.double_drop(null);
        }
    }

    pub mod bounds {
        use std::fmt::Debug;

        pub trait HasArea {
            fn area(&self) -> f64;
        }

        impl HasArea for Rectangle {
            fn area(&self) -> f64 {
                self.height * self.length
            }
        }
        impl HasArea for Triangle {
            fn area(&self) -> f64 {
                self.height * self.length * 0.5
            }
        }

        #[derive(Debug)]
        pub struct Rectangle {
            pub length: f64,
            pub height: f64,
        }

        #[derive(Debug)]
        pub struct Triangle {
            pub length: f64,
            pub height: f64,
        }

        // the generic `T` must implement `Debug`. Regardless
        // of the type this will work properly.
        fn print_debug<T: Debug>(t: &T) {
            println!("{:?}", t);
        }

        pub fn area<T: HasArea>(t: &T) -> f64 {
            t.area()
        }

        #[allow(dead_code)]
        fn usage() {
            let rect = Rectangle {
                length: 3.0,
                height: 4.0,
            };
            let triangle = Triangle {
                length: 3.0,
                height: 4.0,
            };

            print_debug(&rect);
            println!("Area: {}", area(&rect));

            print_debug(&triangle);
            println!("Area: {}", area(&triangle));
        }
    }

    #[allow(dead_code)]
    pub mod empty_bound {
        // A consequence of how bounds ork is that even if a `trait` doesn't inlude any 
        // functionality, you can still use it as a bound. `Eq` and `Copy` are examples of
        // such `traits` from `std` library.
        pub struct Cardinal;
        pub struct BlueJay;
        struct Turkey;

        pub trait Red {}
        pub trait Blue {}

        impl Red for Cardinal {}
        impl Blue for BlueJay {}

        // These function are only valid for types which implement these
        // traits. The fact that the traits are empty is irrelevant.

        pub fn red<T: Red> (_: &T) -> &'static str { "red" }
        pub fn blue<T: Blue> (_: &T) -> &'static str { "blue" }
    }
}

#[cfg(test)]
mod test {
    use test::generics::{bounds::area, empty_bound::{blue, red}};

    use self::generics::{bounds::{Rectangle, Triangle}, empty_bound::{BlueJay, Cardinal}};

    use super::*;

    #[test]
    fn test_bounds() {
        let rect = Rectangle {
            length: 3.0,
            height: 4.0,
        };
        let triangle = Triangle {
            length: 3.0,
            height: 4.0,
        };

        assert_eq!(12.0f64, area(&rect));
        assert_eq!(6.0f64, area(&triangle));
    }

    #[test]
    fn test_empty_bounds() {
        let cardinal = Cardinal;
        let blue_jay = BlueJay;

        assert_eq!("blue", blue(&blue_jay));
        assert_eq!("red", red(&cardinal));
    }
    
}