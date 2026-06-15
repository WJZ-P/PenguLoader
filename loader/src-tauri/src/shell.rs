use std::process::Command;
use std::fs::File;
use std::io;
use std::path::Path;
use zip::ZipArchive;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
fn extract_zip(zip_path: &str, dest_dir: &str) -> Result<(), String> {
    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    
    archive.extract(dest_dir).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn copy_file(src: &str, dest: &str) -> Result<(), String> {
    std::fs::copy(src, dest).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn expand_folder(path: &str) {
    #[cfg(windows)]
    {
        let path = path.replace("/", "\\");
        Command::new("explorer")
            .args(["/expand,", &path])
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "macos")]
    Command::new("open").arg(path).spawn().unwrap();
}

#[tauri::command]
fn reveal_file(path: &str) {
    #[cfg(windows)]
    {
        let path = path.replace("/", "\\");
        Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "macos")]
    Command::new("open").args(["-R", path]).spawn().unwrap();
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("shell")
        .invoke_handler(tauri::generate_handler![expand_folder, reveal_file, extract_zip, copy_file])
        .build()
}
