FROM ubuntu:18.04

COPY ./target/release/raspberry-monitor /bin/raspberry-monitor

CMD ["/bin/raspberry-monitor"]
