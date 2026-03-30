use std::thread;
use std::time::Instant;

use super::world::{World, WORLD_TICK_INTERVAL};

pub fn run_world_loop(mut world: World) {
    let dt = WORLD_TICK_INTERVAL;
    let max_catch_up = 5usize;
    let mut next_tick_deadline = Instant::now() + dt;

    loop {
        let now = Instant::now();
        let mut catch_up_count = 0usize;

        while now >= next_tick_deadline {
            world.tick(next_tick_deadline);
            next_tick_deadline += dt;
            catch_up_count += 1;

            if catch_up_count >= max_catch_up {
                next_tick_deadline = now + dt;
                break;
            }
        }

        let sleep_for = next_tick_deadline.saturating_duration_since(Instant::now());
        if !sleep_for.is_zero() {
            thread::sleep(sleep_for);
        }
    }
}
