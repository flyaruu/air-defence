#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::simulation::run_simulation_reader;

    const DATA: &[u8] = include_bytes!("../../data.csv");

    #[tokio::test]
    async fn integration_test() {
        let c = Cursor::new(DATA);

        let b = BufReader::new(c);
        let statistics = run_simulation_reader(0,100,b).await.unwrap();
        assert_eq!(10,statistics.friendlies_detected);
        assert_eq!(10,statistics.hostile_detected);
        assert_eq!(20,statistics.scans);
        assert_eq!(0,statistics.scan_errors);
    }
}
