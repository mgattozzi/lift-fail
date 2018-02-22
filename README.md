# lift-fail

A library to handle `failure` in an iterable collection.

## About
Sometimes you'll want to do something where a lot of things can fail at once,
usually when doing parallel work. What if some things fail? What if some don't?
What if it's all good? This library helps handle that if you use the `failure`
crate to handle errors.

## Getting Started

Put this in your `Cargo.toml`:

```
[dependencies]
lift-fail = "0.1"
```

Then in your `lib.rs` or `main.rs` file:

```
extern crate lift_fail
```

Then wherever you need it:

```
use lift_fail::lift_fail;
```

Here's a basic use case:

```rust
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
```

## Examples
See the `examples` directory to see how to use the library is used in other ways.

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
