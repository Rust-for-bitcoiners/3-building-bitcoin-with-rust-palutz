#![allow(unused)]

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        match self {
            MResult::Ok(_) => true,
            _ => false
        }
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        match self {
            MResult::Err(_) => true,
            _ => false
        }
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        if let MResult::Ok(t) = self {
            t
        } else {
            panic!("This is not a MResult::Ok")
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        if let MResult::Err(t) = self {
            t
        } else {
            panic!("This is not a MResult::Err")
        }
    }
}

// Add unit tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructors() {
        let res_ok : MResult<i32, &str> = MResult::ok(1);
        let res_ko : MResult<i32, &str> = MResult::Err("Error");

        assert!(res_ok.is_ok());
        assert!(res_ko.is_err());
        assert_eq!(1, res_ok.unwrap());
        assert_eq!("Error", res_ko.unwrap_err());
    }

    #[test]
    #[should_panic]
    fn test_errors() {
        let res_ok : MResult<i32, &str> = MResult::ok(1);
        let res_ko : MResult<i32, &str> = MResult::Err("Error");

        res_ok.unwrap_err();
        res_ko.unwrap();

    }
}
