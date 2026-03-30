use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::state::Connected;

pub fn handle(runtime: &Runtime, connected: Arc<Mutex<Connected>>, connection_id: usize) {
    let responder = runtime.block_on(async { connected.lock().await.get_client(connection_id) });
    if let Some(responder) = responder {
        let _ = runtime.block_on(async move { responder.close_connection().await });
    }
}
