use std::sync::mpsc::{Receiver, Sender};

use log::{info, warn};

use crate::{radar::RadarMessage, IFFMessage};

pub struct IFF {
    radar_receiver: Receiver<RadarMessage>,
    fire_order_sender: Sender<IFFMessage>
}

impl IFF {
    pub fn new(radar_receiver: Receiver<RadarMessage>, fire_order_sender: Sender<IFFMessage>)->Self {
        Self { radar_receiver, fire_order_sender }
    }

    pub fn listen(&self) {
        loop {
            match self.radar_receiver.recv() {
                Ok(msg) => {
                    match msg {
                        RadarMessage::Received(items) => {
                            if is_hostile(items) {
                                self.fire_order_sender.send(IFFMessage::Fire).unwrap();
                            }
                        },
                        RadarMessage::ScanError => {
                            warn!("IFF received error message from radar, ignoring and continuing");
                        },
                        RadarMessage::EndOfData => {
                            info!("Radar reports no more data, shutting down IFF");
                            self.fire_order_sender.send(IFFMessage::IFFShutDown).unwrap();
                        },
                    }
                },
                Err(_) => {
                    warn!("Radar channel was interrupted, shutting down IFF");
                    break;                  
                },
            }
        }
    }
}

fn is_hostile(values: Vec<u8>)->bool {
    let odd_count = values.iter()
        .filter(|value|*value % 2 != 0)
        .count();
    let is_hostile = odd_count << 1 > values.len();
    is_hostile
}

#[cfg(test)]
mod tests {
    use crate::iff::is_hostile;

    #[test]
    fn test_hostile_calc() {
        assert!(!is_hostile(vec![]));
        assert!(!is_hostile(vec![2,4]));
        assert!(!is_hostile(vec![2,5]));
        assert!(is_hostile(vec![1,2,5]));
    }
}