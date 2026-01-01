use std::{clone, thread, time::Duration, error::Error};
use tauri::{AppHandle, Emitter, Manager, App};
use tauri_plugin_global_shortcut::{ Code, Modifiers, Shortcut, ShortcutState, GlobalShortcutExt};
use enigo::{Enigo, Keyboard, Settings, Direction::*, Key};
use arboard::{Clear, Clipboard, Get, Set};
use reqwest::{ get, Client };
use global_hotkey::GlobalHotKeyEvent;



#[derive(serde::Serialize)]
#[derive(Clone)]
struct DefinitionEvent{
    term: String,
    definition: String
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app: &mut tauri::App| {
            let ctrl_shift_d = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyD);

            app.global_shortcut().on_shortcut(ctrl_shift_d, move |app, shortcut, event: GlobalHotKeyEvent | {
                if event.state() == ShortcutState::Pressed {
                    let settings: Settings = Settings { 
                        release_keys_when_dropped: true,
                        ..Default::default()
                };

                    let mut enigo = match Enigo::new(&settings) {
                        Ok(e) => e,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };

                    let mut clip_board = Clipboard::new().unwrap();
                    
                    let prev_text = match clip_board.get_text() {
                        Ok(text) => text,
                        Err(e) => "".to_string()
                    };
                    
                    enigo.key(Key::Meta , Press).unwrap();
                    enigo.key(Key::Unicode('c'), Click).unwrap();
                    enigo.key(Key::Meta, Release).unwrap();
                    
                    thread::sleep(Duration::from_millis(100));

                    let term = clip_board.get_text().unwrap();
                    //TODO------

                    let app_handle = app.clone();
                    let term_clone =  term.clone();
                    println!("{}", term_clone);
                    
                    tauri::async_runtime::spawn(
                   async move {
                        let result:Result<String, reqwest::Error> = async {
                            let text = reqwest::get(format!("http://localhost:3001/defintions/search/{}", term_clone))
                                .await?
                                .text()
                                .await?;
                            Ok(text)
                            }.await;

                            match result {
                                Ok(def) => {
                                    let payload= DefinitionEvent {
                                        term: term_clone.clone(),
                                        definition: def
                                    };
                                    app_handle.emit("defintion_event", payload).unwrap();    
                                },
                                Err(e) => println!("Error, could not get definition")
                            }


                        } 
                    );

                    //TODO------
                    
                    let window = app.get_webview_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        if prev_text == term {
                            window.hide().unwrap();
                        }
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
            }).unwrap();
                


            if cfg!(debug_assertions) { //This part is for logging in debug mode
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
