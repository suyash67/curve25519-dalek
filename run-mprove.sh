# Total estimated time (mins) ~= 8.85
# Note: timings estimated based on Intel® Core™ i7-5500U CPU @ 2.40GHz (on a single core)

cargo build --release
cargo run --release --bin mprove_bin 250 200 -n 1
cargo run --release --bin mprove_bin 500 200 -n 1
cargo run --release --bin mprove_bin 1000 200 -n 1
cargo run --release --bin mprove_bin 2500 200 -n 1
cargo run --release --bin mprove_bin 5000 200 -n 1
cargo run --release --bin mprove_bin 10000 200 -n 1
cargo run --release --bin mprove_bin 25000 200 -n 1
cargo run --release --bin mprove_bin 50000 200 -n 1
cargo run --release --bin mprove_bin 100000 200 -n 1
cargo run --release --bin mprove_bin 250000 200 -n 1

# Total estimated time (mins) ~= 1.41
# Note: timings estimated based on Intel® Core™ i7-5500U CPU @ 2.40GHz (on a single core)

cargo build --release
cargo run --release --bin mprove_bin 10000 50 -n 1
cargo run --release --bin mprove_bin 10000 100 -n 1
cargo run --release --bin mprove_bin 10000 200 -n 1
cargo run --release --bin mprove_bin 10000 500 -n 1
cargo run --release --bin mprove_bin 10000 1000 -n 1
cargo run --release --bin mprove_bin 10000 2000 -n 1
cargo run --release --bin mprove_bin 10000 5000 -n 1