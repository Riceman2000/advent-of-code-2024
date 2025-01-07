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

use regex::Regex;
use std::fs;
use std::path::Path;

#[proc_macro]
pub fn include_year_modules(_input: TokenStream) -> TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = manifest_dir + "/src";
    let src_path = Path::new(&src_dir);

    if !src_path.is_dir() {
        panic!("Provided path is not a directory.");
    }

    let year_module_re: Regex = Regex::new(r"aoc\d+$").unwrap();

    let mut module_stmts = Vec::new();
    for entry in fs::read_dir(src_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_string();

        // Check if the file name matches the regex.
        if year_module_re.is_match(&file_name_str) {
            // Generate a module statement.
            let module_name: syn::Ident = syn::parse_str(&file_name_str).unwrap();
            let stmt = quote! {
                pub mod #module_name;
            };
            module_stmts.push(stmt);
        }
    }

    // Generate and return the token stream.
    let expanded = quote! {
        #(#module_stmts)*
    };
    TokenStream::from(expanded)
}
