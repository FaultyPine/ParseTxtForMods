use hash40::{Hash40, hash40};
use std::collections::HashMap;
use std::io::prelude::*;
use prc::param::*;


/*
becuase prc-rs doesn't support changing what's already in a ParamKind,
make the vector and fill in values first, and then make the ParamKind::Struct 
once that "struct" in the text file is finished

now it does!
https://discordapp.com/channels/447823426061860879/699809178658668615/776354401173504010

*/

// called for each txt file in root (walked recursively)
pub fn path_to_map(path: &std::path::Path) -> Result<HashMap<String, Option<ParamStruct>>, &str> { // { struct name - Option(ParamKind) }
    let mut map: HashMap<String, Option<ParamStruct>> = HashMap::new();
    
    let file_lines_string: String = std::fs::read_to_string(path.to_owned()).unwrap();
    let file_lines: Vec<&str> = file_lines_string[..].split("\n").collect();


    let mut values: Vec<(String, Vec<(Hash40, ParamKind)>)> = Vec::new(); // { struct name - Vec(param_hash, param_kind) }
    for line in file_lines {
        //println!("{}", line);

        if line.contains("= {") { // block start
            let struct_name_str = line[..line.find("= {").unwrap()].trim().to_string();

            // when we find a new struct, start a new vector for that struct
            values.push( (struct_name_str.clone(), Vec::new()) );
 
            //println!("struct name: {}", struct_name_str);
        }
        else if line.contains("}") { // block end
            let current_tup = values.pop().unwrap(); // we definitely want this to panic if the values vec is empty at this point

            map.insert(current_tup.0.clone(), Some(ParamStruct(current_tup.1)));

            //println!("--------------    Finished parsing {}    ---------------", current_tup.0);
        }
        else if !line.trim().is_empty() && line.contains("=") { // value in block
            let current_param: Vec<&str> = line.split(" =").collect();
            let param_name;
            if current_param[0].contains("0x") { // if its a raw hex string
                param_name = hash40::Hash40::from_hex_str(current_param[0]).unwrap_or(hash40!("ERROR"));
            }
            else { // otherwise convert the string to its hash
                param_name = hash40::to_hash40(current_param[0]);
            }
            

            let param_value_str = &current_param[1].trim().replace(",", "")[..];
            let param_value = parse_string_for_param_kind(param_value_str).unwrap_or(ParamKind::Str("ERROR".to_string()));
            
            // push current param value and hash onto the vec assosiated with the most recently pushed vec
            let vec_len = values.len();
            values.get_mut(vec_len-1).unwrap().1.push( (param_name.clone(), param_value.clone()) );

            //println!("param ting: {}, {:#?}", param_name, param_value);

        }
        
    }

    let _param_file_name = path.to_str().unwrap().rsplit("\\").collect::<Vec<&str>>();
    //println!("paramfilename: {}", *(param_file_name.get(0).unwrap()));

    Ok(map)
}


// turn an str into a relevant ParamKind
fn parse_string_for_param_kind(s: &str) -> Option<ParamKind> {
    if let Ok(i) = s.parse() {
        Some(ParamKind::I32(i))
    }
    else if let Ok(f) = s.parse() {
        Some(ParamKind::Float(f))
    }
    else if s == "False" || s == "True" {
        Some(ParamKind::Bool(s.contains("T")))
    }
    else {
        Some(ParamKind::Str(s.to_string()))
    }
}