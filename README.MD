# Raspberry Monitor

This is a pet project intended to create a data logger for useful Raspberry-Pi data, such as CPU temperature.

## Getting started

To simply run it in your RaspberryPi (tested in Raspbian):

```bash
cargo build --release
./target/release/raspberry-monitor
```

It will output a CSV file named `data.csv`.

If you wish to run it in background:

```bash
nohup ./target/release/raspberry-monitor &
```

## Next improvements

- Set output file-name via CLI
- Set sampling rate via CLI
- Add other measurement types
- Generalize `write` method for `StdOutWriter` and `CsvFileWriter`
