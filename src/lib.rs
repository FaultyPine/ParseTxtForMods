#![allow(non_snake_case)]

use std::collections::HashMap;
use std::io::prelude::*;
use hash40::{Hash40, hash40};
use prc::param::ParamKind;
use walkdir::WalkDir;

mod parse;

pub fn txt_to_map(filepath: &str) -> Result<HashMap<String, HashMap<String, ParamKind>>, std::io::Error> {
    
    let mut map = HashMap::new();

    if let Ok(labels) = hash40::read_labels("ParamLabels.csv") {
        hash40::set_labels(labels);
        println!("ParamLabels read successfully");
    } else {
        println!("Failed to read ParamLabels");
    }


    for entry in WalkDir::new(filepath).contents_first(true) {
        let dir_entry = entry?;
        let path = dir_entry.path();
        if path.is_dir() {
            continue;
        }
        let metadata = std::fs::metadata(&path);
        println!("reading {}", path.display());
        map.insert(String::from(path.to_str().unwrap()), parse::path_to_map(path));

    }

    println!("{:?}", map);

    return Ok(map);
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_test() {

        let args: Vec<String> = std::env::args().collect();
        let root = &args[1]; // args[0] is arg after "cargo"
        println!("{}", root);
        match crate::txt_to_map(root) {
            Ok(_) => println!("\nparsed text files at {:?}", std::time::SystemTime::now()),
            Err(e) => println!("failed to parse txt files: {}", e),
        }

    }
}

