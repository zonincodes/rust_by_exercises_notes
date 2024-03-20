pub mod traits {
    pub struct Sheep {
        naked: bool,
        name: &'static str,
    }

    pub trait Animal {
        // Associated function signature; `Self` refers to the implementation type
        fn new(name: &'static str) -> Self;

        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definations.
        fn talk(&self) {
            println!("{} seys {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        pub fn is_naked(&self) -> bool {
            self.naked
        }

        pub fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);
                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        // 'Self` is the implementor typr: `Sheep`.
        fn new(name: &'static str) -> Self {
            Sheep {
                name: name,
                naked: false,
            }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaaah?"
            } else {
                "baaaaaah!"
            }
        }

        // Default trait methods can be overridden.
        fn talk(&self) {
            // Example, we can add some quiet contemplation.
            println!("{} pauses briefly.. {}", self.name, self.noise());
        }
    }
}


#[cfg(test)]
mod test{
    use super::traits::{Animal, Sheep};

    #[test]
    fn test_traits() {
        let  dolly: Sheep = Animal::new("Dolly");

        assert_eq!("baaaaaah!", dolly.noise())
    }
}