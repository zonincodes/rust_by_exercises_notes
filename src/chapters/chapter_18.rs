pub mod error_handling {
    pub mod panic {
        use core::panic;

        pub fn drink(beverage: &str) {
            // You shouldn't drink too much beverages
            if beverage == "lemonade" {
                panic!("AAAaaa!!!")
            }
        }
    }

    pub mod abort_unwind {
        pub fn drink(beverage: &str) -> &'static str {
            if beverage == "lemonade" {
                if cfg!(panic = "abort") {
                    "paniced"
                } else {
                    "Hello"
                }
            } else {
                "drink"
            }
        }
    }

    pub mod options_qmark {
        pub struct Person {
            pub job: Option<Job>,
        }

        #[derive(Clone, Copy)]
        pub struct Job {
            pub phone_number: Option<PhoneNumber>,
        }

        #[derive(Clone, Copy)]
        pub struct PhoneNumber {
            pub area_code: Option<u8>,
            pub number: u32,
        }

        impl Person {
            //Gets the area code of the number of person's job, if it exists.
            pub fn work_phone_area_code(&self) -> Option<u8> {
                self.job?.phone_number?.area_code
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chapters::chapter_18::error_handling::abort_unwind::drink;

    use super::error_handling::{options_qmark::{Person, Job, PhoneNumber}, panic};

    #[test]
    #[should_panic]
    fn test_panics() {
        panic::drink("lemonade")
    }

    #[test]
    fn abort_test() {
        assert_eq!("Hello", drink("lemonade"))
    }

    #[test]
    fn chain_question_mark() {
        let person = Person {
            job: Some(Job { phone_number: Some(PhoneNumber { area_code: Some(69), number: 4206969 }) })
         };

         assert_eq!(69, person.work_phone_area_code().unwrap())
            
    }
}
