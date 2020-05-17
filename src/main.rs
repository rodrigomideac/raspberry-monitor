extern crate chrono;
use chrono::{DateTime, Utc};
use vcgencmd::{measure_temp};
use std::error::Error;
use std::io;
use std::io::Stdout;
use csv::Writer;
use std::fs::File;


#[derive(Debug)]
struct MonitorData{
    date: String,
    temperature: String
}

impl MonitorData{
    fn new() -> MonitorData {
        MonitorData{
            date: get_current_date(),
            temperature: get_current_temperature()
        }
    }

    fn header() -> Vec<String>{
        {
            vec!["date".to_string(),
                 "temperature".to_string()]
        }
    }

    fn to_vec(&self) -> Vec<String>{
        vec![self.date.clone(),
             self.temperature.clone()]
    }
}

fn get_current_date() -> String
{
    let now: DateTime<Utc> = Utc::now();
    format!("{}", now)
}


fn get_current_temperature() -> String
{
    let temp = measure_temp().unwrap();
    format!("{:.2}", temp)
}

trait DataOutput{
    fn init(header: Vec<String>) -> Self;
    fn write(&mut self, data: &MonitorData) -> Result<(), Box<dyn Error>>;
}

struct StdOutWriter{
    wtr: Writer<Stdout>
}

impl DataOutput for StdOutWriter{
    fn init(header: Vec<String>) -> Self {
        let mut wtr =csv::Writer::from_writer(io::stdout());
        wtr.write_record(&header).unwrap();
        StdOutWriter{
            wtr
        }
    }

    fn write(&mut self, data: &MonitorData) -> Result<(), Box<dyn Error>> {
        self.wtr.write_record(&data.to_vec())?;

        self.wtr.flush()?;
        Ok(())
    }
}


struct CsvFileWriter{
    wtr: Writer<File>
}

impl DataOutput for CsvFileWriter{
    fn init(header: Vec<String>) -> Self {
        let mut wtr =csv::Writer::from_path("data.csv").unwrap();
        wtr.write_record(&header).unwrap();
        CsvFileWriter{
            wtr
        }
    }

    fn write(&mut self, data: &MonitorData) -> Result<(), Box<dyn Error>> {
        self.wtr.write_record(&data.to_vec())?;
        self.wtr.flush()?;
        Ok(())
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    use std::{thread, time};

    let ten_seconds = time::Duration::from_secs(10);
    let mut stdout = StdOutWriter::init(MonitorData::header());
    let mut file = CsvFileWriter::init(MonitorData::header());
    loop{
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
