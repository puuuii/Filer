#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{path::Path, fs::{read_dir}, time::SystemTime};

use serde::Serialize;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_content_from_path])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_content_from_path(dir: String) -> Metadata {
  let dir = Path::new(&dir);
  walk_dir(dir)
}

fn walk_dir(dir: &Path) -> Metadata {
  let mut child_metas = Vec::new();
  let read_result = read_dir(dir);
  if dir.is_dir() && read_result.is_ok() {
    for entry in read_result.unwrap() {
      if entry.is_err() { continue };

      let path = entry.unwrap().path();
      let meta_res = path.metadata();
      if meta_res.is_ok() {
        let meta = meta_res.unwrap();
        child_metas.push( Metadata {
          path: path.to_str().unwrap().to_string(),
          is_dir: false,
          is_file: meta.is_file(),
          len: meta.len(),
          modified: meta.modified().unwrap(),
          accessed: meta.accessed().unwrap(),
          created: meta.created().unwrap(),
          childs: vec![],
        });
      }
    }
  }

  match dir.metadata() {
    Ok(meta) => Metadata {
      path: dir.to_str().unwrap().to_string(),
      is_dir: false,
      is_file: meta.is_file(),
      len: meta.len(),
      modified: meta.modified().unwrap(),
      accessed: meta.accessed().unwrap(),
      created: meta.created().unwrap(),
      childs: child_metas,
    },
    _ => Metadata {
        path: dir.to_str().unwrap().to_string(),
        is_dir: false,
        is_file: false,
        len: 0,
        modified: SystemTime::now(),
        accessed: SystemTime::now(),
        created: SystemTime::now(),
        childs: vec![],
    },
  }
}

#[derive(Serialize)]
struct Metadata {
  path: String,
  is_dir: bool,
  is_file: bool,
  // is_symlink: bool,
  len: u64,
  modified: SystemTime,
  accessed: SystemTime,
  created: SystemTime,
  childs: Vec<Metadata>,
}