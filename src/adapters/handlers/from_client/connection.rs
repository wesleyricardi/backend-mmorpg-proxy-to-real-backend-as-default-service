use socket::{Data, Responder};

use crate::adapters::world;
use crate::state::ServerState;

pub async fn on_connect(res: Responder, data: Data<ServerState>) {
    log::trace!("client connected");
    if let Err(_) = data
        .servers_connected
        .lock()
        .await
        .new(res.clone(), data.clone())
        .await
    {
        log::debug!("closing connection with the client");
        if let Err(error) = res.close_connection().await {
            log::error!(
                "fail to close connection with the client, error: {:?}",
                error
            );
        }
    }
}

pub async fn on_disconnect(res: Responder, data: Data<ServerState>) {
    log::trace!("client disconnected");
    world::clear_record_data_bootstrap_for_connection(res.id, std::time::Instant::now()).await;
    data.pending_record_data.lock().await.remove(&res.id);
    world::remove_player_for_connection(data.clone(), res.id).await;
    data.servers_connected.lock().await.remove(res.id).await;
}
