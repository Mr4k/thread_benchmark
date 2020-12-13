all: atomic distributed locking yolo
	@echo "Done"
yolo:
	cd yolo_counter && cargo build --release
atomic:
	cd atomic_counter && cargo build --release
distributed:
	cd distributed_counter && cargo build --release
locking:
	cd locking_counter && cargo build --release
