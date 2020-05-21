use raspberry_monitor::data::MonitorData;
use raspberry_monitor::output::{CsvFileWriter, DataOutput, StdOutWriter};
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    use std::{thread, time};

    let ten_seconds = time::Duration::from_secs(10);
    let mut stdout = StdOutWriter::init(MonitorData::header());
    let mut file = CsvFileWriter::init(MonitorData::header());
    loop {
        let sample = MonitorData::new();
        stdout.write(&sample)?;
        file.write(&sample)?;
        thread::sleep(ten_seconds);
    }

    Ok(())
}

fn main() {
    run();
}
