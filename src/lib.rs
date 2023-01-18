//! This is a small procedural macro to load your `.env` file at compile time. That way you can use
//! [`std::env!`][] to load environment variables and fail the build if a variable is missing.
//!
//! All it does is call the [dotenvy](https://crates.io/crates/dotenvy) crate.
//!
//! # Example
//!
//! `.env` file:
//!
//! ```text
//! KEY=value
//! ```
//!
//! Rust:
//!
//! ```
//! use load_dotenv::load_dotenv;
//!
//! load_dotenv!();
//!
//! fn main() {
//!     assert_eq!("value", env!("KEY"));
//! }
//! ```
//!
//! [`std::env!`]: https://doc.rust-lang.org/std/macro.env.html

#![doc(html_root_url = "https://docs.rs/load-dotenv/0.1.2")]

use proc_macro::TokenStream;

/// Load the `.env` file and panic if the file is missing.
#[proc_macro]
pub fn load_dotenv(_: TokenStream) -> TokenStream {
    dotenvy::dotenv().expect("Failed to load .env file");

    TokenStream::new()
}

/// Load the `.env` file but don't panic if the file is missing.
#[proc_macro]
pub fn try_load_dotenv(_: TokenStream) -> TokenStream {
    dotenvy::dotenv().ok();

    TokenStream::new()
}
