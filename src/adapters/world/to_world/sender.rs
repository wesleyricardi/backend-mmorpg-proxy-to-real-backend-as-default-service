use std::sync::mpsc::Sender;
use std::sync::OnceLock;

use crate::application::use_case::runtime::world_runtime::world_command::WorldCommand;

static WORLD_TX: OnceLock<Sender<WorldCommand>> = OnceLock::new();

pub fn init_world_sender(tx: Sender<WorldCommand>) {
    let _ = WORLD_TX.set(tx);
}

pub fn world_sender() -> Option<&'static Sender<WorldCommand>> {
    WORLD_TX.get()
}
