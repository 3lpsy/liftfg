use fgdb::entity::wrappers::RequestableData;
use tauri::{self, ipc::Invoke, ipc::InvokeBody};
pub mod user;

pub fn parse_body<T>(body: InvokeBody) -> Result<T, String>
where
    T: RequestableData,
{
    match body {
        InvokeBody::Json(json) => serde_json::from_value(json)
            .map_err(|e| format!("Failed to deserialize JSON payload: {}", e)),
        InvokeBody::Raw(bytes) => {
            let s = String::from_utf8(bytes).map_err(|e: std::string::FromUtf8Error| {
                format!("Failed to convert raw bytes to UTF-8: {}", e)
            })?;
            serde_json::from_str(&s)
                .map_err(|e| format!("Failed to deserialize JSON from string: {}", e))
        }
    }
}

pub fn generate<R: tauri::Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![user::create_user]
}
