use std::sync::Arc;

use comfy_table::{
    Cell, ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL,
};
use tokio::{
    sync::{Mutex, broadcast::Receiver},
    task::{AbortHandle, JoinSet},
};

use crate::{
    fire_assessment::FireAssessmentMessage, fire_unit::FireUnitMessage, iff::IFFMessage,
    radar::RadarMessage,
};

#[derive(Debug, Default)]
struct Statistics {
    scans: u64,
    scan_errors: u64,
    friendlies_detected: u64,
    hostile_detected: u64,
    missiles_fired: u64,
    missiles_hit: u64,
    missiles_missed: u64,
}

pub struct Stats {
    statistics: Arc<Mutex<Statistics>>,
}

impl Stats {
    pub fn new() -> Self {
        let statistics = Arc::new(Mutex::new(Statistics::default()));
        Self { statistics }
    }

    pub async fn radar_stats_task(
        &self,
        mut receiver: Receiver<RadarMessage>,
        join_set: &mut JoinSet<()>,
    ) -> AbortHandle {
        let stats = self.statistics.clone();
        join_set.spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                match msg {
                    RadarMessage::Received(_) => {
                        let mut stats = stats.lock().await;
                        stats.scans += 1;
                    }
                    RadarMessage::ScanError => {
                        let mut stats = stats.lock().await;
                        stats.scan_errors += 1;
                    }
                    RadarMessage::EndOfData => {}
                }
            }
        })
    }

    pub async fn iff_stats_task(
        &self,
        mut receiver: Receiver<IFFMessage>,
        join_set: &mut JoinSet<()>,
    ) -> AbortHandle {
        let stats = self.statistics.clone();
        join_set.spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                match msg {
                    IFFMessage::HostileDetected => {
                        let mut stats = stats.lock().await;
                        stats.hostile_detected += 1;
                    }
                    IFFMessage::FriendlyDetected => {
                        let mut stats = stats.lock().await;
                        stats.friendlies_detected += 1;
                    }
                    IFFMessage::IFFShutDown => {}
                }
            }
        })
    }

    pub async fn fireunit_stats_task(
        &self,
        mut receiver: Receiver<FireUnitMessage>,
        join_set: &mut JoinSet<()>,
    ) -> AbortHandle {
        let stats = self.statistics.clone();
        join_set.spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                match msg {
                    FireUnitMessage::MissileFired(_) => {
                        let mut stats = stats.lock().await;
                        stats.missiles_fired += 1;
                    }
                    FireUnitMessage::FireUnitShutdown => todo!(),
                }
            }
        })
    }

    pub async fn fire_assessment_stats_task(
        &self,
        mut receiver: Receiver<FireAssessmentMessage>,
        join_set: &mut JoinSet<()>,
    ) -> AbortHandle {
        let stats = self.statistics.clone();
        join_set.spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                match msg {
                    FireAssessmentMessage::Hit(_) => {
                        let mut stats = stats.lock().await;
                        stats.missiles_hit += 1;
                    }
                    FireAssessmentMessage::Miss(_) => {
                        let mut stats = stats.lock().await;
                        stats.missiles_missed += 1;
                    }
                    FireAssessmentMessage::Shutdown() => {}
                }
            }
        })
    }

    pub async fn display(&self) {
        let stats = self.statistics.lock().await;
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(40)
            // .set_header(vec!["Header1", "Header2", "Header3"])
            .add_row(vec![
                Cell::new("Radar scans"),
                Cell::new(stats.scans.to_string()),
            ])
            .add_row(vec![
                Cell::new("Scan errors"),
                Cell::new(stats.scan_errors.to_string()),
            ])
            .add_row(vec![
                Cell::new("Friendlies detected"),
                Cell::new(stats.friendlies_detected.to_string()),
            ])
            .add_row(vec![
                Cell::new("Hostiles detected"),
                Cell::new(stats.hostile_detected.to_string()),
            ])
            .add_row(vec![
                Cell::new("Missiles fired"),
                Cell::new(stats.missiles_fired.to_string()),
            ])
            .add_row(vec![
                Cell::new("Missiles hit"),
                Cell::new(stats.missiles_hit.to_string()),
            ])
            .add_row(vec![
                Cell::new("Missiles missed"),
                Cell::new(stats.missiles_missed.to_string()),
            ]);

        // Set the default alignment for the third column to right
        println!("{table}");
    }
}
