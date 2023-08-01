pub trait Validatable<E> {
    fn validate(&self) -> Result<(), E>;
}

pub use fn_args_validator_macros::*;

#[cfg(test)]
mod tests {
    use fn_args_validator_macros::with_validation;

    use crate::Validatable;

    type A = i32;
    impl Validatable<bool> for A {
        fn validate(&self) -> Result<(), bool> {
            if *self > 0 {
                Err(false)
            } else {
                Ok(())
            }
        }
    }

    #[with_validation(x, y)]
    fn f(x: A, y: A) -> Result<A, bool> {
        Ok(x + y)
    }

    #[with_validation(z)]
    fn g(z: A) -> Result<A, bool> {
        Ok(z + 1)
    }

    #[test]
    fn test_f() {
        let a = f(-1, -2).unwrap();
        assert!(a == -3);
        let a = f(1, -2);
        assert!(a.is_err());
        assert!(a.err().unwrap() == false);
    }

    #[test]
    fn test_g() {
        let a = g(-1).unwrap();
        assert!(a == 0);
        let a = g(1);
        assert!(a.is_err());
        assert!(a.err().unwrap() == false);
    }
}
