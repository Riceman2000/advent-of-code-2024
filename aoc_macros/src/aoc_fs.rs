use std::path::Path;
use std::sync::LazyLock;
use std::{fs, path::PathBuf};

use regex::Regex;
use syn::PathSegment;

pub static YEAR_MODULE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(aoc\d{4})(\.rs)?$").unwrap());
pub static YEAR_NUMBER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"aoc(\d{4})(\.rs)?$").unwrap());
pub static DAY_MODULE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(day\d\d_\d)(\.rs)?$").unwrap());
pub static DAY_NUMBER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"day(\d\d)_\d(\.rs)?$").unwrap());
pub static DAY_PART_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"day\d\d_(\d)(\.rs)?$").unwrap());

/// Fetch the long input from aoc API and place an empty file for the short input if they are not present
/// Does nothing if they are already there
pub fn fetch_inputs(year: usize, day: usize) {
    let path_long = PathBuf::from(format!("input/{year}/day{day:02}.txt"));
    let path_short = PathBuf::from(format!("input/{year}/day{day:02}-short.txt"));

    // Create year path if necessary
    let year_dir = path_long.parent().expect("Input dir malformed");
    if !year_dir.is_dir() {
        fs::create_dir(year_dir).expect("Could not create year input dir");
    }

    // Place input files
    if !path_short.exists() {
        fs::File::create(path_short).expect("Failed to create short input file");
    }

    // TODO: Get input from API
    if !path_long.exists() {
        fs::File::create(path_long).expect("Failed to create long input file");
    }
}

pub fn get_years() -> Vec<PathBuf> {
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

pub fn get_days(year_path: &Path) -> Vec<PathBuf> {
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

pub fn extract_from_path(path: &Path, re: &Regex) -> String {
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

pub fn get_syn_path(year: &Path, day: &Path) -> syn::Path {
    let year_module = extract_from_path(year, &YEAR_MODULE_RE);
    let year_ident: syn::Ident = syn::parse_str(&year_module).unwrap();
    let year_segment = PathSegment {
        ident: year_ident,
        arguments: syn::PathArguments::None,
    };

    let day_module = extract_from_path(day, &DAY_MODULE_RE);
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
