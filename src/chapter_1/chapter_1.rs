pub mod chapter_1 {
    use core::fmt;


    fn _debug() {
        // This structure cannot be printed either with `fmt::Display` or
        // With  `fmt::Debug`.
        struct UnPrintable(i32);

        // The `derive` attribute automatically creates  yhe implementation
        // required to make this `struct` printable with `fmt::Debug`
        #[derive(Debug)]
        struct DubugPrintable(i32);

        // Std library types are automatically printable with {:?}
    }

    #[derive(Debug)]
    pub struct MinMax(i64, i64);

    impl MinMax {
        pub fn new(x: i64, y: i64) -> MinMax {
            MinMax(x, y)
        }
    }

    // implement `Display` for  `MinMax`

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Use `self.number` to reder to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }


    // Define a structure where the fields are nameable for comparison. 
    #[derive(Debug)]
    pub struct Point2D {
        x: f64,
        y: f64,
    } 

    // Implement init for Point2D
    impl Point2D {
        pub fn new(x: f64, y:f64) -> Point2D{
            Point2D {
                x,
                y,
            }
        }
    }

    // Similary, implement `Display` for `Point2D`

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    impl fmt::Binary for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {:b}, y: {:b}", self.x as i64, self.y as i64)
        }
    }

    //  Test Case List
    // Define a structure named `List` containing a `Vec`.

    pub struct List(pub Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and  create a refernce to `vec`.
            let vec = &self.0;

            write!(f, "[")?;
            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `cout`

            for (count, v) in vec.iter().enumerate() {
                // for every element except the first add, a comma.
                // Use the ? operator to return on erros.
                if count != 0 { write!(f, ", ")?;}
                write!(f, "{}", v)?;
            }

            // Close the oppened bracket and return fmt::Result value
            write!(f, "]")
        }
    }
}

#[cfg(test)]
mod test {
    use super::chapter_1::List;

    #[test]
    fn formart_print() {
        // In general, the `{}` will be automatically replaced with any
        // arguments. This will be stringfied.

        let str = format!("{} days", 31);

        assert_eq!("31 days", str);

        // Positional arguments can be used . Specfiying an integer `{}`
        // Determines which additiona; arguments will be replaced. Areguments starts
        // at 0 immediately adfer the format string.
        let str = format!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
        assert_eq!("Alice, this is Bob. Bob, this is Alice", str);

        // As can named arguments.
        let str: String = format!(
            "{subject} {verb} {object}",
            object = "the lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );

        assert_eq!("the quick brown fox jumps over the lazy dog", str);

        // different formatting can be invocked by specifying the format character
        // after a `:`
        let base_10 = format!("Base 10: {}", 69420);
        let base_2 = format!("Base 2 (binary): {:b}", 69420);
        let base_8 = format!("Base 8 (octal): {:o}", 69420);
        let base_16 = format!("Base 16 (hexadecimal): {:x}", 69420);
        let base_16_x = format!("Base 16 (hexadecimal): {:X}", 69420);

        assert_eq!("Base 10: 69420", base_10);
        assert_eq!("Base 2 (binary): 10000111100101100", base_2);
        assert_eq!("Base 8 (octal): 207454", base_8);
        assert_eq!("Base 16 (hexadecimal): 10f2c", base_16);
        assert_eq!("Base 16 (hexadecimal): 10F2C", base_16_x);

        // You can right justify text with a specified width. This will
        // output "    1". (Four white space and a "1", for a total width of 5.)
        let str = format!("{number:>5}", number = 1);
        assert_eq!("    1", str);

        // you can pad nmbers with extra zeroes,
        let str = format!("{number:0>5}", number = 1);
        assert_eq!("00001", str);

        // You can use named arguments in thr format specifier by appending a '$'
        let str = format!("{number:0>width$}", number = 1, width = 5);
        assert_eq!("00001", str);

        // Rust even checks to make sure the correct number of arguments are used.
        println!("My name is {0}, {1} {0}", "Bond", "James");
        let pi = 3.141592;

        // Add a println! macro call that prints: Pi is roughly 3.142 by controlling
        // the number of decimal places shown. For the purposes of this exercise,
        //use let pi = 3.141592 as an estimate for pi.
        let str = format!("{pi:.3}");
        assert_eq!("3.142", str);
    }

    #[test]
    fn format_display() {
        let v: List = List(vec![1, 2, 3]);
        let str: String = format!("{v}");
        assert_eq!("[1, 2, 3]", str);
    }
}
