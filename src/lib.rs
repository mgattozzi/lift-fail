extern crate failure;

use failure::{ Error, Fail };
use std::fmt;

struct LiftFailure {
    errors: Vec<Error>
}

impl Fail for LiftFailure {}

impl fmt::Debug for LiftFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err_msg = String::new();
        let len = self.errors.len();
        for (idx, err) in self.errors.iter().enumerate() {
            if idx != len - 1 {
                err_msg.push_str(&format!("Error #{}\n{}\n\n", idx + 1, err));
            } else {
                err_msg.push_str(&format!("Error #{}\n{}", idx + 1, err));
            }
        }
        writeln!(f,"{}", err_msg)
    }
}

impl fmt::Display for LiftFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err_msg = String::new();
        let len = self.errors.len();
        for (idx, err) in self.errors.iter().enumerate() {
            if idx != len - 1 {
                err_msg.push_str(&format!("Error #{}\n{}\n\n", idx + 1, err));
            } else {
                err_msg.push_str(&format!("Error #{}\n{}", idx + 1, err));
            }
        }
        writeln!(f,"{}", err_msg)
    }
}

impl LiftFailure {
    fn new() -> Self {
        Self {
            errors: Vec::new()
        }
    }
    fn push(&mut self, input: Error) {
        self.errors.push(input);
    }
    fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

pub fn lift_fail<T,U>(input: T) -> Result<Vec<U>, Error>
    where T: IntoIterator<Item=Result<U, Error>>
{
    let mut output = Vec::new();
    let mut concat = LiftFailure::new();
    for item in input.into_iter() {
        match item {
            Ok(no_fail) => output.push(no_fail),
            Err(e) => concat.push(e),
        }
    }

    if concat.is_empty() {
        Ok(output)
    } else {
        Err(concat.into())
    }
}
