use hash40::{Hash40, hash40};
use std::collections::HashMap;
use std::io::prelude::*;
use prc::param::ParamKind;

// called for each txt file in root (walked recursively)
pub fn path_to_map(path: &std::path::Path) -> HashMap<String, ParamKind> {
    let mut map = HashMap::new();
    
    let buffered = std::io::BufReader::new(std::fs::File::open(path).unwrap());
    let file_lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();
    println!("\n");
    for line in &file_lines {
        println!("{}", line);
    }

    let param_file_name = path.to_str().unwrap().rsplit("\\").collect::<Vec<&str>>();
    println!("paramfilename: {}", *(param_file_name.get(0).unwrap()));

    map.insert(String::from("bruh".to_string()), ParamKind::U16(10));
    map
}