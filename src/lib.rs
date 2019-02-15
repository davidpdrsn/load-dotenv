//! This is a small procedural macro to load your `.env` file at compile time. That way you can use
//! [`std::env!`][] to load environment variables and fail the build if a variable is missing.
//!
//! All it does is call the [dotenv](https://crates.io/crates/dotenv) crate.
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
//! [`std::env!`]: https://doc.rust-lang.org/std/env/fn.vars.html

extern crate proc_macro;

use quote::quote;

/// Load the `.env` file and panic if the file is missing.
#[proc_macro]
pub fn load_dotenv(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv::dotenv().expect("Failed to load .env file");

    (quote! {}).into()
}

/// Load the `.env` file but don't panic if the file is missing.
#[proc_macro]
pub fn try_load_dotenv(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv::dotenv().ok();

    (quote! {}).into()
}
