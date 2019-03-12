./target/release/finn-assembler "$(< ${1})" > output/src/main.rs

cd output
cargo build --release
# cargo rustc --release -- -C target-cpu=native
time ./target/release/output
cd ..