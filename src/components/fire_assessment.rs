use log::{info, warn};
use rand::Rng;
use tokio::sync::broadcast::{Receiver, Sender};

use super::fire_unit::FireUnitMessage;

pub const PK: f32 = 0.8;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FireAssessmentMessage {
    Hit(FireUnitMessage),
    Miss(FireUnitMessage),
    Shutdown(),
}

pub struct FireAssessment {
    fire_unit_receiver: Receiver<FireUnitMessage>,
    fire_assessment_sender: Sender<FireAssessmentMessage>,
}

impl FireAssessment {
    pub fn new(
        fire_unit_receiver: Receiver<FireUnitMessage>,
        fire_assessment_sender: Sender<FireAssessmentMessage>,
    ) -> Self {
        Self {
            fire_unit_receiver,
            fire_assessment_sender,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let more_data = match self.fire_unit_receiver.recv().await {
                Ok(msg) => self.assess_missile(msg).await,
                Err(_) => {
                    info!("Fire unit channel interrupted, shutting down Fire assessment Unit");
                    // self.fire_assessment_sender
                    break;
                }
            };
            if !more_data {
                break;
            }
        }
        info!("Fire assessment completed")
    }

    async fn assess_missile(&mut self, msg: FireUnitMessage) -> bool {
        let value = rand::rng().random::<f32>();
        match &msg {
            FireUnitMessage::MissileFired(_) => {
                if value < PK {
                    info!("... hit!");
                    // Should not unwrap, sending can fail when no receivers are present
                    self.fire_assessment_sender
                        .send(FireAssessmentMessage::Hit(msg))
                        .expect("error sending message");
                } else {
                    info!("... miss!");
                    self.fire_assessment_sender
                        .send(FireAssessmentMessage::Miss(msg))
                        .expect("error sending message");
                }
            }
            FireUnitMessage::FireUnitShutdown => {
                return false;
            }
        }
        true
    }
}
