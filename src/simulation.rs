use std::{fs::File, io::{BufRead, BufReader}, sync::mpsc, thread::spawn};

use crate::{fire_unit::FireUnit, iff::IFF, radar::Radar};

pub fn run_simulation(delay_in_millis: u64, path: &str) {
    let (radar_sender, radar_receiver) = mpsc::channel();
    let reader = BufReader::new(File::open(path).unwrap());
    let text_lines = reader.lines();
    let mut radar = Radar::new(radar_sender, text_lines, delay_in_millis);
    let radar_handle = spawn(move || {
        radar.run();
    });
    let (fire_order_sender, fire_order_receiver) = mpsc::channel();
    let iff = IFF::new(radar_receiver, fire_order_sender);
    let iff_handle = spawn(move || {
        iff.listen();
    });
    let mut fire_unit = FireUnit::new(fire_order_receiver);
    let fire_order_handle = spawn(move || {
        fire_unit.listen();
    });
    
    radar_handle.join().unwrap();
    iff_handle.join().unwrap();
    fire_order_handle.join().unwrap();

}