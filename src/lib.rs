#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::HashMap;
use hash40::{Hash40, hash40};
use prc::param::ParamStruct;
use walkdir::WalkDir;

mod parse;

/*
Returns String (filepath) - (Hashmap of String - ParamKind)
*/

pub fn txt_to_map(filepath: &str) -> Result<HashMap<String, HashMap<String, Option<ParamStruct>>>, &str> {
    
    let mut map = HashMap::new();

    if let Ok(labels) = hash40::read_labels("ParamLabels.csv") {
        hash40::set_labels(labels);
        println!("ParamLabels read successfully");
    } else {
        return Err("Failed to read ParamLabels");
    }


    for entry in WalkDir::new(filepath).contents_first(true) {
        let dir_entry = entry.unwrap();
        let path = dir_entry.path();
        if path.is_dir() {
            //continue; // correct one
            break; // this will ignore stuff outside the fighter folder, using it for better testing
        }
        //let metadata = std::fs::metadata(&path);
        println!("Current file: {}\n", path.display());
        map.insert(String::from(path.to_str().unwrap()), parse::path_to_map(path).unwrap());

    }

    //println!("{:#?}", map);

    Ok(map)
}


pub fn save_map(map: HashMap<String, HashMap<String, Option<ParamStruct>>>) {
    for (k, v) in map.iter() { // { filepath - { struct name - paramstruct(s) } }
        //println!("Key1: {}", k);
        //let mut structs = Vec::new();
        for (k2, v2) in v.iter() {

            //let param_struct: &prc::param::ParamStruct = v2.as_ref().unwrap().try_into_ref().unwrap();
            //let path = &*std::path::Path::new(&k.replace(".txt", "")).join(k2);
            //println!("path: {:#?}", path);
            //println!("paramstruct: {:#?}", param_struct);

            //structs.push(param_struct);
            

        }
        //let structs = structs.into_iter().map(|&paramstruct| paramstruct.unwrap_as_hashmap());
        //prc::save(std::path::Path::new(&k.replace(".txt", ".prc")), final_struct);
    }


}