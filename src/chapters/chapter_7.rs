pub mod loops {
    pub fn match_numbers(number: i32) -> String {
        println!("Tell me about {}", number);
        let result = match number {
            // Match a single value
            1 => String::from("One"),

            // macth several valuess
            2 | 3 | 5 | 11 => String::from("This is a prime"),

            // match an inclusive range
            age @13..=19 => format!("A {age}teen"),

            // Handle the rest of cases
            _ => String::from("Ain't special"),
        };

        result
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_loops() {
        let mut counter = 0;
        'outer: loop {
            counter += 1;
            println!("Enteredd the outer loop");

            loop {
                println!("Entered the inne loop");

                counter += 1;

                if counter == 10 {
                    break 'outer;
                }
            }
        }

        assert_eq!(counter, 10)
    }

    use crate::chapters::chapter_7::loops;
    #[test]
    fn test_macth() {
        let number = 13;

        assert_eq!(loops::match_numbers(3), String::from("This is a prime"));
        assert_eq!(loops::match_numbers(number), String::from("A 13teen"));
        assert_eq!(loops::match_numbers(1), String::from("One"));
        assert_eq!(loops::match_numbers(20), String::from("Ain't special"));
    }
}
