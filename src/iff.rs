
use log::{info, warn};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{IFFMessage, radar::RadarMessage};

pub struct Iff {
    radar_receiver: Receiver<RadarMessage>,
    fire_order_sender: Sender<IFFMessage>,
}

impl Iff {
    pub fn new(
        radar_receiver: Receiver<RadarMessage>,
        fire_order_sender: Sender<IFFMessage>,
    ) -> Self {
        Self {
            radar_receiver,
            fire_order_sender,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            match self.radar_receiver.recv().await {
                Ok(msg) => match msg {
                    RadarMessage::Received(items) => {
                        if is_hostile(items) {
                            self.fire_order_sender.send(IFFMessage::Fire).unwrap();
                        }
                    }
                    RadarMessage::ScanError => {
                        warn!("IFF received error message from radar, ignoring and continuing");
                    }
                    RadarMessage::EndOfData => {
                        info!("Radar reports no more data, shutting down IFF");
                        self.fire_order_sender
                            .send(IFFMessage::IFFShutDown)
                            .unwrap();
                    }
                },
                Err(_) => {
                    warn!("Radar channel was interrupted, shutting down IFF");
                    break;
                }
            }
        }
    }
}

fn is_hostile(values: Vec<u8>) -> bool {
    let odd_count = values.iter().filter(|value| *value % 2 != 0).count();
    
    odd_count << 1 > values.len()
}

#[cfg(test)]
mod tests {
    use crate::iff::is_hostile;

    #[test]
    fn test_hostile_calc() {
        assert!(!is_hostile(vec![]));
        assert!(!is_hostile(vec![2, 4]));
        assert!(!is_hostile(vec![2, 5]));
        assert!(is_hostile(vec![1, 2, 5]));
    }
}
