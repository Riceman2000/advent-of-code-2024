extern crate proc_macro;

use std::path::Path;
use std::sync::LazyLock;
use std::{fs, path::PathBuf};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{parse_macro_input, Expr, PathSegment};

static YEAR_MODULE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(aoc\d{4})(\.rs)?$").unwrap());
static DAY_MODULE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(day\d\d_\d)(\.rs)?$").unwrap());

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

fn get_years() -> Vec<PathBuf> {
    let mut src_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    src_path.push("src");

    assert!(
        src_path.is_dir(),
        "Improper directory structure for src directory"
    );

    let mut out = Vec::new();
    for entry in fs::read_dir(src_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_string();

        // Only accept module directories
        if !entry.path().is_dir() {
            continue;
        }

        // Check if the file name matches the regex.
        if YEAR_MODULE_RE.is_match(&file_name_str) {
            out.push(entry.path());
        }
    }
    out.sort();
    out
}

fn get_days(year_path: &Path) -> Vec<PathBuf> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut search_path = PathBuf::from(manifest_dir);
    search_path.push("src");

    assert!(search_path.is_dir(), "Search directory was not dir");

    // Get path of year module and verify it matches expectations
    let year_file_name_str = year_path.file_name().unwrap().to_string_lossy().to_string();
    let year_module = YEAR_MODULE_RE
        .captures(&year_file_name_str)
        .expect("Macro called from improper file")
        .get(1)
        .expect("Failed to find year module")
        .as_str();
    search_path.push(year_module);

    let mut out = Vec::new();
    for entry in fs::read_dir(search_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read directory entry");
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_string();

        // Check if the file name matches the regex.
        if DAY_MODULE_RE.is_match(&file_name_str) {
            out.push(entry.path());
        }
    }
    out.sort();
    out
}

fn module_from_path(path: &Path, re: &Regex) -> String {
    let file_name_str = path
        .file_name()
        .expect("Failed to read file name")
        .to_string_lossy()
        .to_string();

    re.captures(&file_name_str)
        .expect("File does not match day pattern")
        .get(1)
        .expect("Failed to find day module")
        .as_str()
        .to_string()
}

fn get_syn_path(year: &Path, day: &Path) -> syn::Path {
    let year_module = module_from_path(year, &YEAR_MODULE_RE);
    let year_ident: syn::Ident = syn::parse_str(&year_module).unwrap();
    let year_segment = PathSegment {
        ident: year_ident,
        arguments: syn::PathArguments::None,
    };

    let day_module = module_from_path(day, &DAY_MODULE_RE);
    let day_ident: syn::Ident = syn::parse_str(&day_module).unwrap();
    let day_segment = PathSegment {
        ident: day_ident,
        arguments: syn::PathArguments::None,
    };

    // Construct path from segments
    syn::Path {
        leading_colon: None,
        segments: syn::punctuated::Punctuated::from_iter(vec![year_segment.clone(), day_segment]),
    }
}

#[proc_macro]
pub fn day_process_list(_input: TokenStream) -> TokenStream {
    let mut days = Vec::new();
    for year in get_years() {
        days.extend(get_days(&year));
    }

    let mut module_stmts = Vec::new();
    for year in get_years() {
        let year_module = module_from_path(&year, &YEAR_MODULE_RE);
        for day in get_days(&year) {
            let day_path = get_syn_path(&year, &day);
            let day_str = day_path.to_token_stream().to_string();

            // Remove spaces
            let day_str: String = day_str.chars().filter(|c| *c != ' ').collect();

            let stmt = quote! {
                #[cfg(feature = #year_module)]
                {
                    use #day_path as day;

                    let day_ran = args.target_day.is_empty() || glob_match(&args.target_day, #day_str);
                    if !day_ran {
                        results.push(DayResult {
                            day_name: #day_str,
                            day_ran,
                            passed_test: false,
                            benchmark: None,
                        });
                    }

                    if !args.output_disable && !(args.bench_table || args.bench_graph) {
                        println!("{} -> {}", #day_str, day::day());
                    }

                    // It is best to avoid testing when it wont be reported because it will duplicate user
                    // debug statements
                    let (benchmark, passed_test) = if args.bench_table || args.bench_graph || args.bench_enable {
                        print!("Benchmarking {}.", #day_str);
                        (Some(bench_day(day::day, &args)), day::verify_day(false))
                    } else {
                        (None, false)
                    };

                    results.push(DayResult {
                        day_name: #day_str,
                        day_ran,
                        passed_test,
                        benchmark,
                    });
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
    for year in get_years() {
        days.extend(get_days(&year));
    }

    let mut module_stmts = Vec::new();
    for year in get_years() {
        let year_module = module_from_path(&year, &YEAR_MODULE_RE);
        for day in get_days(&year) {
            let day_module = module_from_path(&day, &DAY_MODULE_RE);
            let day_path = get_syn_path(&year, &day);

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
    for year in get_years() {
        days.extend(get_days(&year));
    }

    let mut module_stmts = Vec::new();
    for year in get_years() {
        let year_module = module_from_path(&year, &YEAR_MODULE_RE);
        for day in get_days(&year) {
            let day_module = module_from_path(&day, &DAY_MODULE_RE);
            let day_path = get_syn_path(&year, &day);

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
