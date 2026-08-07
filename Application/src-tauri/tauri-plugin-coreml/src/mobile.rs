use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_coreml);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Coreml<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_coreml)?;
  Ok(Coreml(handle))
}

/// Access to the coreml APIs.
pub struct Coreml<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Coreml<R> {
  pub fn run_inference(&self, payload: InferenceRequest) -> crate::Result<InferenceResponse> {
    self
      .0
      .run_mobile_plugin("runInference", payload)
      .map_err(Into::into)
  }
}
