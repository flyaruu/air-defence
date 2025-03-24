use std::sync::mpsc::Receiver;

use log::{info, warn};
use rand::Rng;

use crate::IFFMessage;

pub const PK: f32 = 0.8;
pub struct FireUnit {
    fire_order_receiver: Receiver<IFFMessage>,
}

impl FireUnit {
    pub fn new(fire_order_receiver: Receiver<IFFMessage>) -> Self {
        Self {
            fire_order_receiver,
        }
    }

    pub fn listen(&mut self) {
        loop {
            match self.fire_order_receiver.recv() {
                Ok(msg) => match msg {
                    IFFMessage::Fire => fire(),
                    IFFMessage::IFFShutDown => {
                        info!("IFF stream completed, shutting down fire unit");
                        break;
                    }
                },
                Err(_) => {
                    warn!("IFF channel interrupted, shutting down Fire Unit");
                    break;
                }
            }
        }
    }
}

fn fire() {
    info!("Firing missile!");
    let mut rng = rand::rng();
    let value = rng.random::<f32>();
    if value < PK {
        info!("... hit!");
    } else {
        info!("... miss!");
    }
}
