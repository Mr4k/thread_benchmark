all: atomic distributed locking
	@echo "Done"
atomic:
	cd atomic_counter && cargo build --release
distributed:
	cd distributed_counter && cargo build --release
locking:
	cd locking_counter && cargo build --release
