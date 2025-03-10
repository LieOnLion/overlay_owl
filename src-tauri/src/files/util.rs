use std::fs::{self, DirEntry};

#[derive(serde::Serialize)]
pub struct File {
    path: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    extention: Option<String>,
    is_dir: bool,
}

pub fn get_dir_files_as_vec(path: &str) -> Vec<File> {
    let dir = fs::read_dir(path);

    if dir.is_ok() {
        let files_map: Vec<File> = dir
            .ok()
            .unwrap()
            .map(|file| make_file_struct(path, file.unwrap()))
            .collect();
        return files_map;
    }

    return Vec::new();
}

pub fn make_file_struct(path: &str, file: DirEntry) -> File {
    let file_name: String = file.file_name().into_string().unwrap();
    let is_directory: bool = file.metadata().ok().unwrap().is_dir();
    if is_directory {
        return File {
            path: path.to_string(),
            name: file_name,
            extention: Default::default(),
            is_dir: is_directory,
        };
    }
    return File {
        path: path.to_string(),
        name: remove_extention(&file_name),
        extention: Some(get_file_extention(&file_name)),
        is_dir: is_directory,
    };
}

pub fn remove_extention(file_name: &str) -> String {
    if !file_name.contains(".") {
        return file_name.to_string();
    };
    let mut file_name_split: Vec<&str> = file_name.split(".").collect();
    file_name_split.pop();
    return file_name_split.join(".").to_string();
}

pub fn get_file_extention(file_name: &str) -> String {
    if !file_name.contains(".") {
        return "".to_string();
    };
    let file_name_split: Vec<&str> = file_name.split(".").collect();
    return file_name_split.last().unwrap().to_string();
}
