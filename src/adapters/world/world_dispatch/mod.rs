use std::sync::mpsc::Receiver;
use std::sync::Arc;

use tokio::runtime::Builder;
use tokio::sync::Mutex;

use crate::state::Connected;
use crate::world_transport::WorldNetCommand;

pub mod disconnect_client;
pub mod send_to_client;
pub mod send_to_server;

pub fn run_network_dispatcher(rx_net: Receiver<WorldNetCommand>, connected: Arc<Mutex<Connected>>) {
    let Ok(runtime) = Builder::new_current_thread().enable_all().build() else {
        log::error!("failed to build runtime for network_dispatcher");
        return;
    };

    while let Ok(command) = rx_net.recv() {
        super::dispatch_routes::dispatch_command(&runtime, connected.clone(), command);
    }
}
