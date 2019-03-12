./target/release/finn-assembler "$(< ${1})" > test_lang/src/main.rs

cd test_lang
cargo build --release
# cargo rustc --release -- -C target-cpu=native
time ./target/release/test_lang
cd ..