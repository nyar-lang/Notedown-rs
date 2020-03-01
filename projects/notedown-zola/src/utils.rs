use walkdir::{DirEntry, WalkDir};

pub fn filter_files(root: &str) -> Vec<DirEntry> {
    let mut notes = vec![];
    for entry in WalkDir::new(root).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_dir() {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy();
        if file_name.ends_with(".note") | file_name.ends_with(".notedown") {
            notes.push(entry)
        }
    }
    return notes;
}
