
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn aoc_assert(item: TokenStream) -> TokenStream {
    // Parse the input tokens - expected value
    let expected_value = parse_macro_input!(item as Expr);

    let expanded = quote! {
        #[must_use]
        pub fn verify_day(print_output: bool) -> bool {
            let expected = #expected_value;
            let actual = day();

            if actual == expected {
                return true;
            }

            if print_output {
                eprintln!("Got {actual} expected {expected}");
            }
            false
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_day() {
                assert!(verify_day(true));
            }
        }
    };

    TokenStream::from(expanded)
}
