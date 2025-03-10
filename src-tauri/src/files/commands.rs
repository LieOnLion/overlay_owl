use super::util::{get_dir_files_as_vec, File};

#[tauri::command]
pub fn get_dir_stuff(path: &str) -> String {
    let file_names: Vec<File> = get_dir_files_as_vec(path);

    return serde_json::to_string_pretty(&file_names).unwrap();
}
