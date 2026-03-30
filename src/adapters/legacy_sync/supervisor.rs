use socket::Data;
use tokio::time::{sleep, Duration};

use crate::error::AppError;
use crate::state::ServerState;
use crate::unexpected_error;

use super::runtime_sync::{clear_runtime_scope, run_runtime_sync_session};
use super::warmup::{perform_warmup, LEGACY_CONNECT_RETRY_DELAY};

pub async fn supervise_legacy_sync(data: Data<ServerState>) -> Result<(), AppError> {
    log::info!("legacy sync bootstrap phase 1: warmup");
    perform_warmup(data.clone()).await?;
    log::info!("legacy sync bootstrap phase 2: runtime sync");

    match run_runtime_sync_session(data.clone(), true, LEGACY_CONNECT_RETRY_DELAY).await {
        Ok(()) => {
            clear_runtime_scope();
        }
        Err(error) => return Err(error),
    }

    loop {
        log::warn!("legacy runtime sync entering reconnect loop");
        clear_runtime_scope();

        if let Err(error) =
            run_runtime_sync_session(data.clone(), true, LEGACY_CONNECT_RETRY_DELAY).await
        {
            log::error!(
                "legacy runtime sync reconnection session failed: {}. retrying in {:?}",
                error,
                LEGACY_CONNECT_RETRY_DELAY
            );
            clear_runtime_scope();
            sleep(LEGACY_CONNECT_RETRY_DELAY).await;
            continue;
        }
    }
}

pub fn spawn_legacy_sync_supervisor(data: Data<ServerState>) {
    let builder = std::thread::Builder::new()
        .name("game-server-bootstrap".to_string())
        .stack_size(32 * 1024 * 1024);

    if let Err(error) = builder.spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(runtime) => {
                if let Err(error) = runtime.block_on(supervise_legacy_sync(data)) {
                    panic!("{}", error);
                }
            }
            Err(error) => {
                panic!(
                    "{}",
                    unexpected_error!(error, "failed to create bootstrap runtime")
                );
            }
        }
    }) {
        panic!(
            "{}",
            unexpected_error!(error, "failed to spawn game-server-bootstrap thread")
        );
    }
}
