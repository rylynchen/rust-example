use serde::export::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value %2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // try from
    assert_eq!(Ok(EvenNumber(8)), EvenNumber::try_from(8));
    assert_eq!(Err(()), EvenNumber::try_from(5));

    // try into
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(Ok(EvenNumber(8)), result);
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(Err(()), result);
}
