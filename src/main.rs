use std::sync::Arc;

use models::machine_model::AtmMachine;
use tokio::sync::mpsc::channel;

mod machine;
mod models;

fn main() {
    let (tx, rx) = channel::<String>(255);

    tokio::spawn(async move {
        let machine = AtmMachine::new();
    });
}
