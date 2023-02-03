use std::env;
use std::fs;
// enum ArgumentTypes {

// }
fn read_dir(dir: fs::ReadDir) {
    for d in dir {
        if let Ok(result) = d {
            if result.file_type().unwrap().is_dir() {
                if let Ok(next_path) = fs::read_dir(result.path().as_path()) {
                    println!("");
                    println!("Changing directory");
                    println!("");
                    read_dir(next_path)
                } else {
                    println!("Could not read directory")
                };
            } else {
                let file = result.path().as_os_str().to_str().unwrap();
                println!("\t{}", result.path().as_os_str().to_str().unwrap());
            }
        } else {
            println!("Failed to read dir")
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let pattern = match args.get(1) {
        Some(text) => text,
        None => panic!("You did not set a pattern to look for"),
    };

    let location = match env::current_dir() {
        Ok(result) => result,
        Err(_) => panic!("Failed to get current directory"),
    };
    match fs::read_dir(location) {
        Ok(result) => read_dir(result),
        Err(_) => panic!("Failed to read directory"),
    };
}
