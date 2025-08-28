// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tauri::ipc::Response;

const MIN_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// 表示文件信息
#[derive(Serialize, Clone)]
struct FileEntry {
    path: PathBuf,
    size: u64,
}

/// 获取文件逻辑大小（字节数）
fn file_logical_size(path: &Path) -> u64 {
    match fs::metadata(path) {
        Ok(meta) => meta.len(),
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                // 跳过拒绝访问的文件
                0
            } else {
                // 其他错误直接报错
                eprintln!("无法访问文件 {:?}: {}", path, e);
                0
            }
        }
    }
}

/// 递归收集文件
fn scan(path: &Path, files: &mut Vec<FileEntry>) -> io::Result<()> {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                if let Err(e) = scan(&entry_path, files) {
                                    if e.kind() == ErrorKind::PermissionDenied {
                                        eprintln!("跳过目录 {:?}: {}", entry_path, e);
                                        continue;
                                    } else {
                                        return Err(e);
                                    }
                                }
                            } else if entry_path.is_file() {
                                let size = file_logical_size(&entry_path);
                                if size >= MIN_FILE_SIZE {
                                    files.push(FileEntry {
                                        path: entry_path,
                                        size,
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            if e.kind() == ErrorKind::PermissionDenied {
                                eprintln!("跳过无法访问的目录项: {}", e);
                                continue;
                            } else {
                                return Err(e);
                            }
                        }
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                eprintln!("跳过目录 {:?}: {}", path, e);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn scan_and_save(root: &Path) -> io::Result<()> {
    let mut files: Vec<FileEntry> = Vec::new();
    if let Err(e) = scan(&root, &mut files) {
        eprintln!("Error Occured while scanning: {}", e);
    }
    let json = serde_json::to_string_pretty(&files).unwrap();
    fs::write("files.json", json)?;

    println!("已保存文件信息到 files.json (共 {} 个文件)", files.len());
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fetch_files() -> Response {
    let data = std::fs::read("files.json").unwrap();
    Response::new(data)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
