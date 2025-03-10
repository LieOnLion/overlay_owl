use super::dir_finder::{get_dir_files_as_vec, remove_extention};

#[tauri::command]
pub fn get_dir_stuff(path: &str) -> String {
    let file_names: Vec<String> = get_dir_files_as_vec(path)
        .iter()
        .map(|file_name: &String| remove_extention(file_name))
        .filter(|file_name_without_extention: &String| !file_name_without_extention.is_empty())
        .collect();

    return serde_json::to_string_pretty(&file_names).unwrap();
}
