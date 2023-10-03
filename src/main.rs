use std::env;
use std::path::Path;
use std::fs;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}


fn new() {


    let cur_path = get_current_working_dir();

    let new_path = format!("{}/.rvcs", cur_path);



    if Path::new(&new_path).exists() {
        println!("RVCS aleady exists");
        return;
    }

    let temp = fs::create_dir(new_path);

    match temp {
        Ok(_) => println!("RVCS Initialized"),
        Err(_) => println!("Error creating RVCS")
    }




}

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "new" {
        new();
    } else if &args[1] == "commit" {
        println!("build");
    }

    println!("args: {:?}", args);
}
