all:
	cargo build --release
	docker build -t raspberry-monitor .
	docker rm --force raspberry-monitor
	docker run --name raspberry-monitor -it raspberry-monitor:latest /bin/bash
