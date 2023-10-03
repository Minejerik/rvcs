use std::env;
use std::path::Path;
use std::fs;
use std::io::Write;

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn rvcs_exists() -> bool {
    let cur_path = get_current_working_dir();

    let new_path = format!("{}/.rvcs", cur_path);

    Path::new(&new_path).exists()
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

    let mut file = fs::File::create(format!("{}/.rvcs/TRACKED", cur_path)).unwrap();

    file.write_all(b"").unwrap();

    let _ = fs::create_dir(format!("{}/.rvcs/objects", cur_path));
   
}

fn get_rvcs_path() -> String {
    let cur_path = get_current_working_dir();

    let new_path = format!("{}/.rvcs", cur_path);

    new_path
}

fn add_file(file: &String) {
    if rvcs_exists() == false {
        println!("RVCS not initialized");
        println!("Run 'rvcs new' to initialize");
        return;
    }

    let rvcs_path = get_rvcs_path();


    let mut temp_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/TRACKED", rvcs_path))
        .unwrap();

    temp_file.write(format!("{}\n", file).as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "new" {
        new();
    } else if &args[1] == "commit" {
        println!("build");
    } else if &args[1] == "add"{
        add_file(&args[2]);
    }

}
