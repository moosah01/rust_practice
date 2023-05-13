use std::error::Error;
use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub struct ErrorVec<E>(pub Vec<E>);


impl<E: Display> Display for ErrorVec<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, error) in self.0.iter().enumerate() {
            writeln!(f, "Error {}: {}", i + 1, error)?;
        }
        Ok(())
    }
}

impl<E: Display + Error> Error for ErrorVec<E> {}

pub fn collect_errors<T, E>(input: Vec<Result<T, E>>) -> Result<Vec<T>, ErrorVec<E>>
where
    E: Error + Display,
{
    let mut ok_values = Vec::new();
    let mut errors = Vec::new();

    for item in input {
        match item {
            Ok(value) => ok_values.push(value),
            Err(error) => errors.push(error),
        }
    }

    if errors.is_empty() {
        Ok(ok_values)
    } else {
        Err(ErrorVec(errors))
    }
}

fn cause_error() -> Result<i32, io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "an error"))
}

fn main() {
    let values: Vec<Result<i32, io::Error>> = vec![
        Ok(1),
        cause_error(),
        Ok(2),
        cause_error(),
    ];

    match collect_errors(values) {
        Ok(numbers) => println!("Numbers: {:?}", numbers),
        Err(errors) => println!("Errors: {}", errors),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_errors() {
        let values = vec![
            Ok(1),
            Err(io::Error::new(io::ErrorKind::Other, "an error")),
            Ok(2),
            Err(io::Error::new(io::ErrorKind::Other, "another error")),
        ];

        match collect_errors(values) {
            Ok(_) => panic!("Expected error, got Ok"),
            Err(errors) => {
                assert_eq!(errors.0.len(), 2);
                assert_eq!(format!("{}", errors.0[0]), "an error");
                assert_eq!(format!("{}", errors.0[1]), "another error");
            }
        }
    }
}