use std::env;
use std::path::Path;
use std::fs;



fn new() {
    if Path::new("/.rvcs").exists() {
        println!("RVCS aleady exists");
        return;
    }

    let _ = fs::create_dir("/.rvcs");

    


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
