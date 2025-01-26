use tauri::{self, ipc::Invoke};

#[tauri::command]
async fn get_user() -> Result<(), String> {
    Ok(())
}

pub fn generate<R: tauri::Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![get_user]
}
