extern crate proc_macro;

mod aoc_fs;

use std::{
    fs,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

use crate::aoc_fs::{DAY_MODULE_RE, DAY_NUMBER_RE, DAY_PART_RE, YEAR_MODULE_RE, YEAR_NUMBER_RE};

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
    let Some(parent) = file.parent() else {
        // If the macro panics it will mess up rust-analyzer and it has a hard time recovering
        // So just fudge it and do nothing if the file structure is not as anticipated
        eprintln!("Called from improper file");
        return quote! {}.into();
    };

    // Things based on file structure
    let year_number: usize = aoc_fs::extract_from_path(parent, &YEAR_NUMBER_RE)
        .parse()
        .expect("Year number parse failure");
    let day_number: usize = aoc_fs::extract_from_path(&file, &DAY_NUMBER_RE)
        .parse()
        .expect("Day number parse failure");
    let day_part: usize = aoc_fs::extract_from_path(&file, &DAY_PART_RE)
        .parse()
        .expect("Day part parse failure");
    let feature = aoc_fs::extract_from_path(parent, &YEAR_MODULE_RE);

    // Fetch inputs and place blank files
    aoc_fs::fetch_inputs(year_number, day_number);
    let (input_short, input_long) = aoc_fs::input_filenames(year_number, day_number);

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

    // Generate unit tests if user provides expected values and input data exists
    let unit_tests = unit_tests(
        &expected_short,
        &expected_long,
        &PathBuf::from(input_short.clone()),
        &PathBuf::from(input_long.clone()),
    );

    quote! {
    pub static INPUT_SHORT: std::sync::LazyLock<Vec<u8>> =
        std::sync::LazyLock::new(|| std::fs::read(#input_short).expect("Failed to read short input"));
    pub static INPUT_LONG: std::sync::LazyLock<Vec<u8>> =
        std::sync::LazyLock::new(|| std::fs::read(#input_long).expect("Failed to read long input"));

    impl #impl_generics crate::AocDay for #struct_ident #ty_generics #where_clause {
        fn day(input: &'static [u8]) -> Self::OutputType {
            day(input)
        }

        fn day_number() -> usize {
            #day_number
        }
        fn day_part() -> usize {
            #day_part
        }
        fn year_number() -> usize {
            #year_number
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

    #[cfg(feature = #feature)]
    #unit_tests
    }
    .into()
}

fn unit_tests(
    expected_short: &syn::Expr,
    expected_long: &syn::Expr,
    input_short: &Path,
    input_long: &Path,
) -> proc_macro2::TokenStream {
    // If an expected value is not given or the input is missing, a unit test is not created
    let short_test = if is_expr_none(expected_short)
        || !input_short.is_file()
        || fs::metadata(input_short).unwrap().len() == 0
    {
        quote! {}
    } else {
        quote! {
            #[test]
            fn test_short() {
                if Day::input_short().is_empty() {
                    panic!("Short input is empty");
                }
                assert!(Day::verify_short(true));
            }
        }
    };

    let long_test = if is_expr_none(expected_long)
        || !input_long.is_file()
        || fs::metadata(input_long).unwrap().len() == 0
    {
        quote! {}
    } else {
        quote! {
            #[test]
            fn test_long() {
                if Day::input_long().is_empty() {
                    panic!("Long input is empty");
                }
                assert!(Day::verify_long(true));
            }
        }
    };

    quote! {
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::AocDay;

        #short_test

        #long_test
    }
    }
}

fn is_expr_none(expr: &syn::Expr) -> bool {
    // Pattern match against the expression to check if it's a Path type and represents `None`.
    if let syn::Expr::Path(expr_path) = expr {
        // Check if the path is a single identifier (no colons, meaning no module path)
        // and the identifier is "None".
        expr_path.path.segments.len() == 1 && expr_path.path.segments[0].ident == "None"
    } else {
        // If the expression is not a Path type, it's not a `None`.
        false
    }
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
                    let _ = divan::black_box(aoc::#day_path::Day::day_long());
                }
            };

            module_stmts.push(stmt);
        }
    }

    // Generate and return the token stream.
    let expanded = quote! {
        use aoc::AocDay;
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
                _c.bench_function(#fn_name, |b| b.iter(&mut aoc::#day_path::Day::day_long));
            };

            module_stmts.push(stmt);
        }
    }

    // Generate and return the token stream.
    let expanded = quote! {
        use aoc::AocDay;
        #(#module_stmts)*
    };
    TokenStream::from(expanded)
}
