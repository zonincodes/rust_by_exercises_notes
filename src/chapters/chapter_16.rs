pub mod traits {

    pub mod implementation {
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

            pub fn _shear(&mut self) {
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

    pub mod iterators {
        struct Fibonacci {
            curr: u32,
            next: u32,
        }

        // Implement `Iterator` for `Fibinacci`.
        // The `Iterator` trait only requires a method to be defined for the 'next' element.
        impl Iterator for Fibonacci {
            // We can refer to this type using Self::Item
            type Item = u32;
            //to this type of using `.curr`   anf `.next`.

            // The return type is `Option<T>`:
            //      * When the `Iterator` is finished, `None` is returned.
            //      * Otherwise, the nexed value is wrapped in 'Some' and returned.
            // We use Self::Item in the return type, so we can change
            // the type without having to update the funtion signatures.

            fn next(&mut self) -> Option<Self::Item> {
                let current = self.curr;

                self.curr = self.next;
                self.next = current + self.next;

                Some(current)
            }
        }

        pub fn fibonacci() -> Fibonacci {
            Fibonacci {curr: 0, next: 1}
        }

    }
}

#[cfg(test)]
mod test {
    use super::traits::implementation::{Animal, Sheep};

    #[test]
    fn test_traits() {
        let dolly: Sheep = Animal::new("Dolly");

        assert_eq!("baaaaaah!", dolly.noise())
    }
}
