use std::fs::{self};

pub fn get_dir_files_as_vec(path: &str) -> Vec<String> {
    let dir = fs::read_dir(path);

    if dir.is_ok() {
        let files_map: Vec<String> = dir
            .ok()
            .unwrap()
            .map(|file| file.unwrap().file_name().into_string().unwrap())
            .collect();
        return files_map;
    }

    return Vec::new();
}

pub fn remove_extention(file_name: &str) -> String {
    if !file_name.contains(".") {
        return file_name.to_string();
    };
    let mut file_name_split: Vec<&str> = file_name.split(".").collect();
    file_name_split.pop();
    return file_name_split.join(".").to_string();
}
