extern crate proc_macro;

mod aoc_fs;

use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

use crate::aoc_fs::{DAY_MODULE_RE, DAY_NAME_RE, YEAR_MODULE_RE, YEAR_NAME_RE};

#[derive(Default)]
struct AocDayArrts {
    output_type: Option<syn::Type>,

    expected_long: Option<syn::Expr>,
    expected_short: Option<syn::Expr>,
}

/// Macro used to derive the necessary benchmarking and testing functions for an Advent of Code Day
/// # Panics
/// If you use it wrong
#[proc_macro_derive(AocDay, attributes(output_type, expected_short, expected_long))]
pub fn derive_aoc_day(item: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident: struct_ident,
        data,
        generics,
        ..
    }: DeriveInput = parse_macro_input!(item as DeriveInput);

    // Get calling file
    let span = proc_macro::Span::call_site();
    let file = PathBuf::from(span.file());
    let parent = file.parent().expect("Called from improper file");

    // Input file paths
    let year = aoc_fs::extract_from_path(&parent, &YEAR_NAME_RE);
    let day = aoc_fs::extract_from_path(&file, &DAY_NAME_RE);
    let input_short = format!("./input/{year}/{day}-short.txt");
    let input_long = format!("./input/{year}/{day}.txt");

    // Day name
    let day_mod = aoc_fs::extract_from_path(&file, &DAY_MODULE_RE);
    let day_name = format!("{year}::{day_mod}");

    // Feature
    let feature = aoc_fs::extract_from_path(&parent, &YEAR_MODULE_RE);

    // Only accept unit structs
    let syn::Data::Struct(syn::DataStruct {
        struct_token: _,
        fields,
        semi_token: _,
    }) = data
    else {
        panic!("AocDay can only be derived on structs")
    };
    assert!(
        matches!(fields, syn::Fields::Unit),
        "AocDay can only be derived on unit structs"
    );

    // Process attrs
    let mut aoc_day_attrs = AocDayArrts::default();
    for attr in attrs {
        if attr.path().is_ident("output_type") {
            let output_type: syn::LitStr = attr
                .parse_args()
                .expect("Expected str literal for output type");
            let output_type = syn::parse_str(&output_type.value())
                .expect("Could not parse output_type into Type");
            aoc_day_attrs.output_type = Some(output_type);
        } else if attr.path().is_ident("expected_long") {
            let expected_long = attr.parse_args().unwrap();
            aoc_day_attrs.expected_long = Some(expected_long);
        } else if attr.path().is_ident("expected_short") {
            let expected_short = attr.parse_args().unwrap();
            aoc_day_attrs.expected_short = Some(expected_short);
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Enforce required values and set defaults for optional values
    let output_type = aoc_day_attrs.output_type.expect("output_type not provided");
    let expected_short = aoc_day_attrs.expected_short.unwrap_or(parse_quote!(None));
    let expected_long = aoc_day_attrs.expected_long.unwrap_or(parse_quote!(None));

    // If an expected value is not given a unit test is not created
    let short_test = if expected_short == parse_quote!(None) {
        quote! {}
    } else {
        quote! {
            #[test]
            fn test_short() {
                assert!(Day::verify_short(true));
            }
        }
    };

    let long_test = if expected_long == parse_quote!(None) {
        quote! {}
    } else {
        quote! {
            #[test]
            fn test_long() {
                assert!(Day::verify_long(true));
            }
        }
    };

    quote! {
    pub static INPUT_SHORT: std::sync::LazyLock<Vec<u8>> =
        std::sync::LazyLock::new(|| std::fs::read(#input_short).expect("Failed to read short input"));
    pub static INPUT_LONG: std::sync::LazyLock<Vec<u8>> =
        std::sync::LazyLock::new(|| std::fs::read(#input_long).expect("Failed to read long input"));

    impl #impl_generics crate::AocDay for #struct_ident #ty_generics #where_clause {
        fn day(input: &'static [u8]) -> Self::OutputType {
            day(input)
        }

        fn name() -> &'static str {
            #day_name
        }

        fn input_long() -> &'static [u8] {
            &INPUT_LONG
        }
        fn input_short() -> &'static [u8] {
            &INPUT_SHORT
        }

        type OutputType = #output_type;
        fn expected_short() -> Option<Self::OutputType> {
            #expected_short
        }
        fn expected_long() -> Option<Self::OutputType> {
            #expected_long
        }
    }

    #[cfg(test)]
    #[cfg(feature = #feature)]
    mod tests {
        use super::*;
        use crate::AocDay;

        #short_test

        #long_test
    }
    }
    .into()
}

#[proc_macro]
pub fn day_process_list(_input: TokenStream) -> TokenStream {
    let mut days = Vec::new();
    for year in aoc_fs::get_years() {
        days.extend(aoc_fs::get_days(&year));
    }

    let mut module_stmts = Vec::new();
    for year in aoc_fs::get_years() {
        let year_module = aoc_fs::extract_from_path(&year, &YEAR_MODULE_RE);
        for day in aoc_fs::get_days(&year) {
            let day_path = aoc_fs::get_syn_path(&year, &day);

            let stmt = quote! {
                #[cfg(feature = #year_module)]
                {
                    results.push(#day_path::Day::process_day());
                }
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

#[proc_macro]
#[allow(clippy::missing_panics_doc)]
pub fn divan_process_list(_input: TokenStream) -> TokenStream {
    let mut days = Vec::new();
    for year in aoc_fs::get_years() {
        days.extend(aoc_fs::get_days(&year));
    }

    let mut module_stmts = Vec::new();
    for year in aoc_fs::get_years() {
        let year_module = aoc_fs::extract_from_path(&year, &YEAR_MODULE_RE);
        for day in aoc_fs::get_days(&year) {
            let day_module = aoc_fs::extract_from_path(&day, &DAY_MODULE_RE);
            let day_path = aoc_fs::get_syn_path(&year, &day);

            // Get function name based off year and day
            let fn_name = format!("{year_module}_{day_module}");
            let fn_ident: syn::Ident = syn::parse_str(&fn_name).unwrap();

            let stmt = quote! {
                #[cfg(all(feature = #year_module, feature = "divan"))]
                #[divan::bench]
                fn #fn_ident() {
                    let _ = divan::black_box(aoc::#day_path::day());
                }
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

#[proc_macro]
pub fn criterion_process_list(_input: TokenStream) -> TokenStream {
    let mut days = Vec::new();
    for year in aoc_fs::get_years() {
        days.extend(aoc_fs::get_days(&year));
    }

    let mut module_stmts = Vec::new();
    for year in aoc_fs::get_years() {
        let year_module = aoc_fs::extract_from_path(&year, &YEAR_MODULE_RE);
        for day in aoc_fs::get_days(&year) {
            let day_module = aoc_fs::extract_from_path(&day, &DAY_MODULE_RE);
            let day_path = aoc_fs::get_syn_path(&year, &day);

            // Get function name based off year and day
            let fn_name = format!("{year_module}_{day_module}");

            let stmt = quote! {
                #[cfg(all(feature = #year_module, feature = "criterion"))]
                _c.bench_function(#fn_name, |b| b.iter(&mut aoc::#day_path::day));
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
