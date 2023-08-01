use fn_args_validator::{with_validation, Validatable};

#[derive(Debug)]
enum Error {
    OutOfRange,
}

type RangedInt = i32;

impl Validatable<Error> for RangedInt {
    fn validate(&self) -> Result<(), Error> {
        if *self > 0 && *self < 100 {
            Ok(())
        } else {
            Err(Error::OutOfRange)
        }
    }
}

#[with_validation(x, y)]
fn f(x: RangedInt, y: RangedInt) -> Result<RangedInt, Error> {
    Ok(x + y)
}

pub fn main() {
    println!("{}", f(1, 2).unwrap());
    println!("{}", f(120, 2).expect("range")); // panic
}
