pub mod chapter_6 {
    #[derive(Debug, PartialEq)]
    pub struct EvenNumber(pub i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::chapters::chapter_6::chapter_6::EvenNumber;

    #[test]
    fn try_from_test() {
        // Try from
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // Try into

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }
}
