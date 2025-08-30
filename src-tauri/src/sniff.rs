// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path};
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tauri::ipc::Response;
use tauri::{AppHandle, ipc::Channel};

const MIN_FILE_SIZE: u64 = 10 * 1024 * 1024;




/// 表示文件信息
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content="data")]
pub enum SniffEvent {
    Found {
        file_path: String,
        file_size: usize,
    }
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
/*
/// 递归收集文件
fn collect_files(path: &Path, files: &mut Vec<FileEntry>) -> io::Result<()> {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                if let Err(e) = collect_files(&entry_path, files) {
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
 */

fn sniff(root: String, on_event:&Channel<SniffEvent>) {
    let root_path = PathBuf::from(root);
    if root_path.is_dir() {
        match fs::read_dir(&root_path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                let entry_str = entry_path.to_string_lossy().into_owned();
                                sniff(entry_str, on_event);
                            } else if entry_path.is_file() {
                                let size = file_logical_size(&entry_path);
                                if size >= MIN_FILE_SIZE {
                                    println!("Found: {}", entry_path.to_string_lossy());
                                    on_event.send(SniffEvent::Found{
                                        file_path: entry_path.to_string_lossy().into_owned(),
                                        file_size:size.try_into().unwrap()}).unwrap();
                                }
                            }
                        }
                        Err(e) => {
                            if e.kind() == ErrorKind::PermissionDenied {
                                eprintln!("跳过无法访问的目录项: {}", e);
                                continue;
                            } else {
                            }
                        }
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                eprintln!("跳过目录 {:?}: {}", root_path, e);
            }
            Err(e) => {
                eprintln!("跳过目录 {:?}: {}", root_path, e);
            }
        }
    }
}

#[tauri::command]
pub async fn recursive_search(on_event:Channel<SniffEvent>) {
    sniff("C:\\Users\\20692\\Documents".to_string(), &on_event);
}