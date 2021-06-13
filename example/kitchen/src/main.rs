use log::{error, info};
use std::{thread, time::Duration};

fn sleep(seconds: f32) {
    thread::sleep(Duration::from_secs_f32(seconds));
}

pub mod dad {
    use super::{info, sleep};

    pub fn cook_spaghetti() -> bool {
        info!("Cooking the spaghetti...");
        sleep(4.0);
        info!("Spaghetti is ready!");
        true
    }
}

pub mod mom {
    use super::{info, sleep};

    pub fn cook_sauce_and_set_table() {
        sleep(1.0);
        info!("Cooking the sauce...");
        sleep(2.0);
        info!("Sauce is ready! Setting the table...");
        sleep(2.0);
        info!("Table is set!");
    }
}

fn main() {
    env_logger::init();
    let handle = thread::spawn(|| dad::cook_spaghetti());

    mom::cook_sauce_and_set_table();

    if handle.join().unwrap_or(false) {
        info!("Spaghetti time! Yum!")
    } else {
        error!("Dad messed up the spaghetti. Order pizza instead?");
    }
}
