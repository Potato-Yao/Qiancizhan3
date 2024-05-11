// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, io};

use tauri::Manager;

use crate::file_util::{get_or_create_resource_dir, get_wordbooks};

mod file_util;
mod db_util;
mod word_model;
mod word_builder;
mod word_util;

fn main() -> Result<(), io::Error> {
    run_app()
}

fn run_app() -> Result<(), io::Error> {
    let resource_path = get_or_create_resource_dir()?;

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_wordbooks,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[cfg(test)]
mod tests {}
