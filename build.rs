use regex::Regex;
use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
};

fn main() {
    // Output file paths
    let out_dir = env::var("OUT_DIR").unwrap();
    let main_day_list_path = out_dir.clone() + "/main_day_list.gen.rs";
    let divan_day_list_path = out_dir + "/divan_day_list.gen.rs";

    // Source paths
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = manifest_dir + "/src";

    // Regex matches for source files and directories
    let years_dir_re: Regex = Regex::new(r"aoc\d+$").unwrap();
    let day_source_re: Regex = Regex::new(r"day\d{2}_(0|1).rs$").unwrap();

    // Find year directories within src
    let src_dir_reader = fs::read_dir(src_dir.clone()).unwrap();
    let mut years = HashMap::new();
    for f in src_dir_reader {
        let entry = f.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let path_str = path.to_string_lossy().to_string();

        if years_dir_re.is_match(&path_str) {
            years.entry(path).or_insert(Vec::new());
        }
    }

    // Find day files within each year directory
    for (year_path, days) in &mut years {
        let year_dir_reader = fs::read_dir(year_path).unwrap();
        for f in year_dir_reader {
            let entry = f.unwrap();
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let path_str = path.to_string_lossy().to_string();

            if day_source_re.is_match(&path_str) {
                days.push(path);
            }
        }
    }

    // Form strings that can later be used in source files
    // Note that some are constructed to be a single block with {}
    let mut main_day_list = String::from("{\n");
    let mut divan_day_list = String::new();
    let mut year_lib_map = HashMap::new();
    for (year_path, days) in years {
        let year = year_path.file_name().unwrap().to_string_lossy().to_string();
        let mut days: Vec<_> = days
            .iter()
            .map(|d| {
                d.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .strip_suffix(".rs")
                    .unwrap()
                    .to_string()
            })
            .collect();
        days.sort();

        let mut year_lib_list = String::from("// Auto generated, no need to edit\n");
        for day in days {
            main_day_list.push_str(&format!("process_day!({year}::{day}, args, results);\n"));
            divan_day_list.push_str(&format!(
                "#[divan::bench]\
                fn {year}_{day}() {{ let _ = black_box({year}::{day}::day()); }} \n"
            ));
            year_lib_list.push_str(&format!("pub mod {day};\n"));
        }
        year_lib_map.entry(year).or_insert(year_lib_list);
    }
    main_day_list.push('}');

    // Write out files to be included
    let mut main_day_list_file = fs::File::create(main_day_list_path).unwrap();
    main_day_list_file
        .write_all(main_day_list.as_bytes())
        .unwrap();

    let mut divan_day_list_file = fs::File::create(divan_day_list_path).unwrap();
    divan_day_list_file
        .write_all(divan_day_list.as_bytes())
        .unwrap();

    for (year, content) in year_lib_map {
        let year_lib_path = format!("{src_dir}/{year}.rs");

        // Some auto running checkers could endlessly loop if we always overwrite this file so read
        // it first to see if changes need to be made
        if let Ok(mut year_lib_file) = fs::File::open(&year_lib_path) {
            let mut current_content = String::new();
            year_lib_file.read_to_string(&mut current_content).unwrap();
            if current_content == content {
                continue;
            }
        }

        let mut year_lib_file = fs::File::create(year_lib_path).unwrap();
        year_lib_file.write_all(content.as_bytes()).unwrap();
    }
}
