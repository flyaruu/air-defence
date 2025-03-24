use std::time::Duration;

use log::info;
use tokio::{sync::broadcast::Sender, time::sleep};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RadarMessage {
    Received(Vec<u8>),
    ScanError,
    EndOfData,
}

pub struct Radar<S> {
    sender: Sender<RadarMessage>,
    lines: S, //Lines<BufReader<File>>,
    delay_in_millis: u64,
}

impl<S: Iterator<Item = Result<String, std::io::Error>>> Radar<S> {
    pub fn new(sender: Sender<RadarMessage>, source: S, delay_in_millis: u64) -> Self {
        info!("Starting radar");
        Self {
            sender,
            lines: source,
            delay_in_millis,
        }
    }
    /// Emits a value returns true if the last data has been emitted, false otherwise
    pub fn emit(&mut self) -> bool {
        let mut end_of_data = false;
        let message = match self.lines.next() {
            Some(line) => match line {
                Ok(line) => parse_line(&line),
                Err(_error) => RadarMessage::ScanError,
            },
            None => {
                end_of_data = true;
                RadarMessage::EndOfData
            }
        };
        self.sender.send(message).unwrap();
        end_of_data
    }

    pub async fn run(&mut self) {
        while !self.emit() {
            info!("Tick!");
            sleep(Duration::from_millis(self.delay_in_millis)).await;
        }
    }
}

fn parse_line(line: &str) -> RadarMessage {
    let items = line
        .split(";")
        .map(|item| item.trim())
        .map(|item| u8::from_str_radix(item, 2).unwrap())
        .collect();
    RadarMessage::Received(items)
}

#[cfg(test)]
mod tests {
    use crate::radar::{RadarMessage, parse_line};

    #[test]
    fn test_parser() {
        assert_eq!(parse_line("1000;1001"), RadarMessage::Received(vec![8, 9]))
    }
}
