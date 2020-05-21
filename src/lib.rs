pub mod data {
    use chrono::{DateTime, Utc};
    use vcgencmd::measure_temp;

    #[derive(Debug)]
    pub struct MonitorData {
        date: String,
        temperature: String,
    }

    impl Default for MonitorData {
        fn default() -> Self {
            MonitorData::new()
        }
    }

    impl MonitorData {
        pub fn new() -> MonitorData {
            MonitorData {
                date: get_current_date(),
                temperature: get_current_temperature(),
            }
        }

        pub fn header() -> Vec<String> {
            {
                vec!["date".to_string(), "temperature".to_string()]
            }
        }

        pub fn to_vec(&self) -> Vec<String> {
            vec![self.date.clone(), self.temperature.clone()]
        }
    }

    pub fn get_current_date() -> String {
        let now: DateTime<Utc> = Utc::now();
        format!("{}", now)
    }

    pub fn get_current_temperature() -> String {
        let temp = measure_temp().unwrap();
        format!("{:.2}", temp)
    }
}

pub mod output {
    use crate::data::MonitorData;
    use csv::Writer;
    use std::error::Error;
    use std::fs::File;
    use std::io;
    use std::io::Stdout;

    pub trait DataOutput {
        fn init(header: Vec<String>) -> Self;
        fn write(&mut self, data: &MonitorData) -> Result<(), Box<dyn Error>>;
    }

    pub struct StdOutWriter {
        wtr: Writer<Stdout>,
    }

    impl DataOutput for StdOutWriter {
        fn init(header: Vec<String>) -> Self {
            let mut wtr = csv::Writer::from_writer(io::stdout());
            wtr.write_record(&header).unwrap();
            StdOutWriter { wtr }
        }

        fn write(&mut self, data: &MonitorData) -> Result<(), Box<dyn Error>> {
            self.wtr.write_record(&data.to_vec())?;

            self.wtr.flush()?;
            Ok(())
        }
    }

    pub struct CsvFileWriter {
        wtr: Writer<File>,
    }

    impl DataOutput for CsvFileWriter {
        fn init(header: Vec<String>) -> Self {
            let mut wtr = csv::Writer::from_path("data.csv").unwrap();
            wtr.write_record(&header).unwrap();
            CsvFileWriter { wtr }
        }

        fn write(&mut self, data: &MonitorData) -> Result<(), Box<dyn Error>> {
            self.wtr.write_record(&data.to_vec())?;
            self.wtr.flush()?;
            Ok(())
        }
    }
}
