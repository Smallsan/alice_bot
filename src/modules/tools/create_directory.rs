use std::fs;

pub fn create_directory(directory_name: &str) {
    if fs::metadata(directory_name).is_ok() {
        println!("Directory {} Already Exists", directory_name);
        return;
    }
    fs::create_dir(directory_name).expect("Error creating directory");
    println!("Directory {} Has Been Created", directory_name);
}
