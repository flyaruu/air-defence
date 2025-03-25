use log::{info, warn};
use tokio::sync::broadcast::{Receiver, Sender};

use super::iff::IFFMessage;

pub struct FireUnit {
    iff_message_receiver: Receiver<IFFMessage>,
    fire_unit_sender: Sender<FireUnitMessage>,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FireUnitMessage {
    MissileFired(IFFMessage),
    FireUnitShutdown,
}

impl FireUnit {
    pub fn new(
        iff_message_receiver: Receiver<IFFMessage>,
        fire_unit_sender: Sender<FireUnitMessage>,
    ) -> Self {
        Self {
            iff_message_receiver,
            fire_unit_sender,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            match self.iff_message_receiver.recv().await {
                Ok(msg) => match msg {
                    IFFMessage::HostileDetected => self.fire(msg).await,
                    IFFMessage::FriendlyDetected => {}
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

    async fn fire(&self, iff_message: IFFMessage) {
        info!("Firing missile!");
        self.fire_unit_sender
            .send(FireUnitMessage::MissileFired(iff_message))
            .unwrap();
    }
}
