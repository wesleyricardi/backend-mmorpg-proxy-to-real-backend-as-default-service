use std::sync::Arc;

use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::state::Connected;
use crate::world_transport::ServerWorldMessage;

pub fn handle(
    runtime: &Runtime,
    connected: Arc<Mutex<Connected>>,
    connection_id: usize,
    message: ServerWorldMessage,
) {
    let responder = runtime.block_on(async { connected.lock().await.get_server(connection_id) });
    if let Some(responder) = responder {
        let _ = runtime.block_on(async move { responder.send(message).await });
    }
}
