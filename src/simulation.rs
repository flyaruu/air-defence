use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use tokio::task::JoinSet;

use crate::{
    fire_assessment::FireAssessment, fire_unit::FireUnit, iff::Iff, radar::Radar, stats::Stats,
};

pub async fn run_simulation(delay_in_millis: u64, path: &str) {
    let reader = BufReader::new(File::open(path).unwrap());
    let text_lines = reader.lines();
    let mut join_set = JoinSet::new();

    let stats = Stats::new();

    let (radar_sender, radar_receiver) = tokio::sync::broadcast::channel(255); // mpsc::channel();
    let radar_stats_receiver = radar_sender.subscribe();
    let mut radar = Radar::new(radar_sender, text_lines, delay_in_millis);
    join_set.spawn(async move {
        radar.run().await;
    });
    let (iff_message_sender, fire_order_receiver) = tokio::sync::broadcast::channel(255);
    let iff_stats_receiver = iff_message_sender.subscribe();

    let mut iff = Iff::new(radar_receiver, iff_message_sender);
    join_set.spawn(async move {
        iff.listen().await;
    });

    let (fireunit_sender, fireunit_receiver) = tokio::sync::broadcast::channel(255);
    let fireunit_stats_receiver = fireunit_sender.subscribe();

    let mut fire_unit = FireUnit::new(fire_order_receiver, fireunit_sender);
    join_set.spawn(async move {
        fire_unit.listen().await;
    });

    let (fire_assessment_sender, fire_assessment_receiver) = tokio::sync::broadcast::channel(256);

    let mut fire_assessment = FireAssessment::new(fireunit_receiver, fire_assessment_sender);
    join_set.spawn(async move {
        fire_assessment.listen().await;
    });

    // let radar_statistics = statistics.clone();

    stats
        .radar_stats_task(radar_stats_receiver, &mut join_set)
        .await;
    stats
        .iff_stats_task(iff_stats_receiver, &mut join_set)
        .await;
    stats
        .fireunit_stats_task(fireunit_stats_receiver, &mut join_set)
        .await;
    stats
        .fire_assessment_stats_task(fire_assessment_receiver, &mut join_set)
        .await;
    join_set.join_all().await;
    // radar_handle.join().unwrap();
    // iff_handle.join().unwrap();
    // fire_order_handle.join().unwrap();
    stats.display().await;
    // info!("Stats: scans: {}",stats.statistics.lock().await.scans)
}
