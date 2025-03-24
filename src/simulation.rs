use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use tokio::task::JoinSet;

use crate::{fire_unit::FireUnit, iff::Iff, radar::Radar};

pub async fn run_simulation(delay_in_millis: u64, path: &str) {
    let (radar_sender, radar_receiver) = tokio::sync::broadcast::channel(255); // mpsc::channel();
    let reader = BufReader::new(File::open(path).unwrap());
    let text_lines = reader.lines();
    let mut radar = Radar::new(radar_sender, text_lines, delay_in_millis);
    let mut join_set = JoinSet::new();
    join_set.spawn(async move {
        radar.run().await;
    });
    let (fire_order_sender, fire_order_receiver) = tokio::sync::broadcast::channel(255);
    let mut iff = Iff::new(radar_receiver, fire_order_sender);
    join_set.spawn(async move {
        iff.listen().await;
    });
    let mut fire_unit = FireUnit::new(fire_order_receiver);
    join_set.spawn(async move {
        fire_unit.listen().await;
    });

    join_set.join_all().await;
    // radar_handle.join().unwrap();
    // iff_handle.join().unwrap();
    // fire_order_handle.join().unwrap();
}
