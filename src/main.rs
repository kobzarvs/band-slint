#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::settings::SETTINGS;
use crate::slint_utils::slint_invoke;
use mattermost_api_rust_driver::api;
use once_cell::sync::Lazy;
use slint::{SharedString, Weak};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

mod settings;
mod slint_utils;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_get_users_me(handle_get_users_me(ui.as_weak()));

    ui.run()?;

    Ok(())
}

fn handle_get_users_me(weak_ui: Weak<AppWindow>) -> impl Fn() {
    move || {
        let api_client = Arc::clone(&API_CLIENT);

        slint_invoke(weak_ui.clone(), move |ui| {
            ui.set_message(SharedString::from("Loading..."));
        });

        let weak_ui_clone = weak_ui.clone();
        // Выполняем асинхронный запрос
        tokio::spawn(async move {
            let api = api_client.lock().await;
            match api.get_users_me().await {
                Ok(user) => {
                    // Обработка успешного результата
                    println!("User ID: {}", user.id);
                    println!("Username: {}", user.username);
                    println!("Email: {}", user.email);
                    slint_invoke(weak_ui_clone, move |ui| {
                        ui.set_message(SharedString::from(user.email));
                    });
                }
                Err(e) => {
                    // Обработка ошибки
                    eprintln!("{}", e);
                    slint_invoke(weak_ui_clone, move |ui| {
                        ui.set_error_message(SharedString::from(format!("{}", e)));
                    });
                }
            }
        });
    }
}

// Глобальный синглтон ApiClient
pub static API_CLIENT: Lazy<Arc<Mutex<api::ApiClient>>> = Lazy::new(|| {
    let client =
        api::ApiClient::new(SETTINGS.server.base_url.clone()).expect("Failed to create API Client");
    Arc::new(Mutex::new(client))
});
