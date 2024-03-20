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

        pub fn red<T: Red>(_: &T) -> &'static str {
            "red"
        }
        pub fn blue<T: Blue>(_: &T) -> &'static str {
            "blue"
        }
    }

    #[allow(dead_code)]
    pub mod multiple_bounds {
        // Multiple bounds for a single type can be applied with a `+`.
        // like normal, differen typed are ser=perated with , .

        use std::fmt::{Debug, Display};

        pub fn compare_prints<T: Debug + Display>(t: &T) {
            println!("Debug: {:?}", t);
            println!("Display: {}", t);
        }

        pub fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
            println!("t: {:?}", t);
            println!("u: {:?}", u);
        }

        pub fn consume() {
            let string = "words";
            let array = [1, 2, 3];
            let vec = vec![1, 2, 3];

            compare_prints(&string);
            compare_types(&array, &vec);
        }
    }

    pub mod where_clouse {
        // A bound can also be expressed using a `where` clause immediately,
        // before the opening `{`, rather than at the type's first mention.
        // Additionally, `where` clauses can applly bounds to arbitrary types, rather
        // than just to type parameters

        trait TraitB {}
        trait TraitC {}
        trait TraitD {}

        trait TraitE {}

        trait TraitF {}
        trait MyTrait<A, D> {}

        struct YourType;

        // Long
        // impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

        // Expressing bounds with a `where` clause
        impl<A, D> MyTrait<A, D> for YourType
        where
            A: TraitB + TraitC,
            D: TraitE + TraitF,
        {
        }
    }

    pub mod the_problem {
        struct Container(i32, i32);

        // a trait which checks if 2 items are stored inside of container.
        // Also retrives first or last value.

        trait Contains<A, B> {
            fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
            fn first(&self) -> i32; // doesn't explicitly require `A` or `B`
            fn last(&self) -> i32; // doesn't explicitly require `A` or `B`
        }

        impl Contains<i32, i32> for Container {
            // True if the numbers stored are equal.
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }

            fn first(&self) -> i32 {
                self.0
            }

            fn last(&self) -> i32 {
                self.1
            }
        }

        // `C` contains `A` and `B`. In light of that, having to express `A` and
        // `B` again is a nuisance

        fn difference<A, B, C>(container: &C) -> i32
        where
            C: Contains<A, B>,
        {
            container.last() - container.first()
        }

        pub fn the_problem() {
            let number_1 = 3;
            let number_2 = 10;

            let container = Container(number_1, number_2);

            println!(
                "Does container contain {} and {}: {}",
                &number_1,
                &number_2,
                container.contains(&number_1, &number_2)
            );
            println!("First number: {}", container.first());
            println!("Last number: {}", container.last());

            println!("The difference is: {}", difference(&container));
        }
    }

    pub mod associated_types {

        struct Container(i32, i32);

        // A trait which checks if 2 items are stored inside of container.
        // Also retrieves first or last value.
        trait Contains {
            // Define generic types here which methods will be able to utilize.
            type A;
            type B;

            fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
            fn first(&self) -> i32;
            fn last(&self) -> i32;
        }

        impl Contains for Container {
            // Specify what types `A` and `B` are. If the `input` type
            // is `Container(i32, i32)`, the `output` types are determined
            // as `i32` and `i32`.
            type A = i32;
            type B = i32;

            // `&Self::A` and `&Self::B` are also valid here.
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }
            // Grab the first number.
            fn first(&self) -> i32 {
                self.0
            }

            // Grab the last number.
            fn last(&self) -> i32 {
                self.1
            }
        }

        fn difference<C: Contains>(container: &C) -> i32 {
            container.last() - container.first()
        }

        pub fn associated_types() {
            let number_1 = 3;
            let number_2 = 10;

            let container: Container = Container(number_1, number_2);

            println!(
                "Does this container contan {}  and {}: {}",
                &number_1,
                &number_2,
                container.contains(&number_1, &number_2)
            )
        }
    }

    mod phantom_type_parameters {
        // This doesn't show up at runtime, but is checked statically
        // (and only) at compile time
        use std::marker::PhantomData;

        // A phantom tuple struct which is generic over `A` with hidden parameter `B`.
        #[derive(PartialEq)] // Allow equality test for this type
        struct PhantomTuple<A, B>(A, PhantomData<B>);

        // A phantom type struct which is generic over `A` with hidden parameter `B`.
        #[derive(PartialEq)] // Allow equality test for this type
        struct PhantomStruct<A, B> {
            first: A,
            phantom: PhantomData<B>,
        }

        fn phantom_type_parameters() {
            // Here `f32` and `f34` are the hidden parameters.
            // PhantomTuple type specified as `char, f32`.
            let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
            // PhantomTuple type specified as `<char, f64>`.
            let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

            // Type specified as `<char, f32>`.
            let _struct1: PhantomStruct<char, f32> = PhantomStruct {
                first: 'Q',
                phantom: PhantomData,
            };

            // Tpe specifed as `<char, f64>`.
            let _struct2: PhantomStruct<char, f64> = PhantomStruct {
                first: 'Q',
                phantom: PhantomData,
            };

            // Compile-time Error! Type mismatch so these cannot be compared:
            // println!("_tuple1 == _tuple2 yields: {}",
            //           _tuple1 == _tuple2);

            // Compile-time Error! Type mismatch so these cannot be compared:
            // println!("_struct1 == _struct2 yields: {}",
            //           _struct1 == _struct2);

            pub mod unit_clarification {
                use std::marker::PhantomData;
                use std::ops::Add;

                /// Create void enumerations to define unit types.
                #[derive(Debug, Clone, Copy)]
                enum Inch {}
                #[derive(Debug, Clone, Copy)]
                enum Mm {}

                /// `Length` is a type with phantom type parameter `Unit`,
                /// and is not generic over length type (that is `f64`).
                ///
                /// `f64` already implements the `Clone` and `Copy` traits
                #[derive(Debug, Clone, Copy)]
                struct Length<Unit>(f64, PhantomData<Unit>);

                /// The `Add` trait defines the behavior of the `+` operator.
                impl<Unit> Add for Length<Unit> {
                    type Output = Length<Unit>;
                    // add() returns a new `Length` Struct containing the sum.
                    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
                        Length(self.0 + rhs.0, PhantomData)
                    }
                }

                fn unit_clarification() {
                    let one_foot: Length<Inch> = Length(12.0, PhantomData);
                    // `one_meter` has phantom type parameter `Mm`.
                    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

                    // `+` calls the `add()` method we implemented for `Length<Unit>`.
                    //
                    // Since `Length` implements `Copy`, `add()` does not consume
                    // `one_foot` and `one_meter` but copies them into `self` and `rhs`.
                    let two_feet = one_foot + one_foot;
                    let two_meters = one_meter + one_meter;

                    // Addition works.
                    println!("one foot + one_foot = {:?} in", two_feet.0);
                    println!("one meter + one_meter = {:?} mm", two_meters.0);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use test::generics::{
        bounds::area,
        empty_bound::{blue, red},
    };

    use self::generics::{
        bounds::{Rectangle, Triangle},
        empty_bound::{BlueJay, Cardinal},
    };

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
