#[macro_use] extern crate failure;
extern crate lift_fail;

use lift_fail::lift_fail;

fn main() {
    let example_vec = vec![
        Ok(1),
        Err(format_err!("Uh oh")),
        Ok(2),
        Err(format_err!("Oh no")),
    ];

    match lift_fail(example_vec) {
        // We have errors in our vec so we know we won't get a vector of just the
        // correct values. We're lifting up the errors into one.
        Ok(_) => unreachable!(),
        Err(e) => println!("{}", e.cause()),
    }
}
