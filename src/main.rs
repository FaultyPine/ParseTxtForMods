#![allow(non_snake_case)]
#![allow(unused_imports)]


pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Make sure to specify the root directory of your changes folder (with a relative path). \n(I.E.  cargo run ../changes)");
        return;
    }
    let root = &args[1];
    println!("Root: {}", root);
    let now = std::time::SystemTime::now();
    match ParseTxtForMods::txt_to_map(root) {
        Ok(map) => {
            println!("\nFiles parsed successsfully!\nTime elapsed: {:#?}", now.elapsed().unwrap());
            println!("Saving param changes...\n");
            let now = std::time::SystemTime::now();
            //ParseTxtForMods::save_map(map);
            println!("Saved params successfully. Time elapsed: {:#?}", now.elapsed().unwrap());
        },
        Err(e) => println!("failed to parse txt files: {}", e),
    }
}