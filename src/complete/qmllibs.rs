mod pluginsgenerate;
use once_cell::sync::Lazy;

use std::sync::Arc;
use tokio::sync::Mutex;
pub static ROOT_LIB_DIR: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new("/usr/lib/qml".to_string())));

pub async fn update_root_dir<S: ToString>(path: S) {
    let mut data = ROOT_LIB_DIR.lock().await;
    *data = path.to_string()
}
