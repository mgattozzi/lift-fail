extern crate failure;
extern crate lift_fail;

use lift_fail::lift_fail;

fn main() {
    let example_vec = vec![
        Ok(1),
        Ok(2),
        Ok(3),
        Ok(4),
    ];

    match lift_fail(example_vec) {
        // All the values are good so we collect them all
        Ok(v) => assert_eq!(v, vec![1, 2, 3, 4]),
        Err(e) => println!("{}", e.cause()),
    }
}
