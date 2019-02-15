# load-dotenv

This is a small procedural macro to load your `.env` file at compile time. That way you can use
[`std::env!`][] to load environment variables and fail the build if a variable is missing.

All it does is call the [dotenv](https://crates.io/crates/dotenv) crate.

## Example

`.env` file:

```
KEY=value
```

Rust:

```rust
use load_dotenv::load_dotenv;

load_dotenv!();

fn main() {
    assert_eq!("value", env!("KEY"));
}
```

[`std::env!`]: https://doc.rust-lang.org/std/macro.env.html

License: MIT
