#![allow(warnings)]

mod adapters;
mod application;
mod config;
mod debug_ui;
mod domain;
mod error;
mod libs;
mod protocol;
mod proxy;
mod state;
mod world_transport;

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use config::{global::LISTEN_PORT, handlers::handlers_config};
use socket::{Server, ServiceBuilder};
use state::get_app_state;

use crate::adapters::{legacy_sync::supervisor::spawn_legacy_sync_supervisor, world};
use crate::application::use_case::runtime::world_runtime::world::World;
use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;
use crate::application::use_case::runtime::world_runtime::world_loop::run_world_loop;
use crate::application::use_case::load_skill_state::load_skill_state_from_files;
use crate::domain::user_player::dto::PlayerJob;
use crate::state::ServerState;
use crate::world_transport::WorldNetCommand;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::log::setup_logger()?;
    debug_ui::init(config::global::debug_ui_enabled());

    log::debug!("------- STARTING SERVER -------");
    let base = PathBuf::from("config").join("Skill");

    let skill_state = load_skill_state_from_files(vec![
        (PlayerJob::Mechanician, base.join("Mech.json")),
        (PlayerJob::Fighter, base.join("Fighter.json")),
        (PlayerJob::Pikeman, base.join("Pike.json")),
        (PlayerJob::Archer, base.join("Archer.json")),
        (PlayerJob::Knight, base.join("Knight.json")),
        (PlayerJob::Atalanta, base.join("Atalanta.json")),
        (PlayerJob::Priestess, base.join("Sacer.json")),
        (PlayerJob::Magician, base.join("Mage.json")),
        (PlayerJob::Assassine, base.join("Assassin.json")),
        (PlayerJob::Shaman, base.join("Shaman.json")),
        (PlayerJob::Martial, base.join("Martial.json")),
    ])?;

    let server_state = get_app_state(skill_state.clone()).await;

    let (tx_world, rx_world) = mpsc::channel::<WorldCommand>();
    let (tx_net, rx_net) = mpsc::channel::<WorldNetCommand>();
    world::init_world_sender(tx_world);

    let world = World::new(skill_state, rx_world, tx_net);
    thread::spawn(move || run_world_loop(world));

    let connected = server_state.servers_connected.clone();
    thread::spawn(move || world::run_network_dispatcher(rx_net, connected));

    spawn_legacy_sync_supervisor(server_state.clone());

    let listen_addr = format!("0.0.0.0:{LISTEN_PORT}");
    let mut services = ServiceBuilder::new();
    services
        .app_data(server_state)
        .channel_capacity(4096)
        .service_concurrency(2048)
        .configure(handlers_config);

    Server::new(services).bind(listen_addr).run().await;

    Ok(())
}
