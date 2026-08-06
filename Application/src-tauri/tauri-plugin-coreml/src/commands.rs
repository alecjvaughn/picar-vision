use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::CoremlExt;

#[command]
pub(crate) async fn run_inference<R: Runtime>(
    app: AppHandle<R>,
    payload: InferenceRequest,
) -> Result<InferenceResponse> {
    app.coreml().run_inference(payload)
}
