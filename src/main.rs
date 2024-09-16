#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![deny(clippy::restriction)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::needless_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::print_stdout)]
#![allow(clippy::exit)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::min_ident_chars)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::separated_literal_suffix)]
#![allow(clippy::modulo_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::integer_division_remainder_used)]
#![allow(clippy::self_named_module_files)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::panic)]
#![allow(clippy::expect_used)]
#![allow(clippy::use_debug)]

use utils::generate_readme;

mod best_expressions;
mod char;
mod constants;
mod expression;
mod expression_generator;
mod utils;

fn main() {
    generate_readme();
}
