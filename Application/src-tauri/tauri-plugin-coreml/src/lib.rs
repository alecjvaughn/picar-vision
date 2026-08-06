use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Coreml;
#[cfg(mobile)]
use mobile::Coreml;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the coreml APIs.
pub trait CoremlExt<R: Runtime> {
  fn coreml(&self) -> &Coreml<R>;
}

impl<R: Runtime, T: Manager<R>> crate::CoremlExt<R> for T {
  fn coreml(&self) -> &Coreml<R> {
    self.state::<Coreml<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("coreml")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let coreml = mobile::init(app, api)?;
      #[cfg(desktop)]
      let coreml = desktop::init(app, api)?;
      app.manage(coreml);
      Ok(())
    })
    .build()
}
