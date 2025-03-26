use log::{info, warn};
use tokio::sync::broadcast::{Receiver, Sender};

use super::radar::RadarMessage;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum IFFMessage {
    HostileDetected(RadarMessage),
    FriendlyDetected(RadarMessage),
    IFFShutDown,
}
pub struct Iff {
    radar_receiver: Receiver<RadarMessage>,
    iff_message_sender: Sender<IFFMessage>,
}

impl Iff {
    pub fn new(
        radar_receiver: Receiver<RadarMessage>,
        iff_message_sender: Sender<IFFMessage>,
    ) -> Self {
        Self {
            radar_receiver,
            iff_message_sender,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            match self.radar_receiver.recv().await {
                Ok(msg) => match &msg {
                    RadarMessage::Received(items) => {
                        if is_hostile(items) {
                            self.iff_message_sender
                                .send(IFFMessage::HostileDetected(msg))
                                .expect("error sending message");
                        } else {
                            self.iff_message_sender
                                .send(IFFMessage::FriendlyDetected(msg))
                                .expect("error sending message");
                        }
                    }
                    RadarMessage::ScanIOError(error_msg) => {
                        warn!(
                            "IFF received error message: {} from radar, ignoring and continuing",
                            error_msg
                        );
                    }
                    RadarMessage::ScanIntError(parse_int_error) => {
                        warn!(
                            "IFF received parsing message: {:?} from radar, ignoring and continuing",
                            parse_int_error
                        );
                    }

                    RadarMessage::EndOfData => {
                        info!("Radar reports no more data, shutting down IFF");
                        self.iff_message_sender
                            .send(IFFMessage::IFFShutDown)
                            .expect("error sending message");
                    }
                },
                Err(_) => {
                    info!("Radar channel was interrupted, shutting down IFF");
                    break;
                }
            }
        }
    }
}

/// Actual calculation
fn is_hostile(values: &Vec<u8>) -> bool {
    let odd_count = values.iter().filter(|value| *value % 2 != 0).count();

    odd_count << 1 > values.len()
}

#[cfg(test)]
mod tests {
    use crate::components::iff::is_hostile;

    #[test]
    /// Test a few IFF scenarios
    fn test_hostile_calc() {
        assert!(!is_hostile(&vec![]));
        assert!(!is_hostile(&vec![2, 4]));
        assert!(!is_hostile(&vec![2, 5]));
        assert!(is_hostile(&vec![1, 2, 5]));
    }
}
