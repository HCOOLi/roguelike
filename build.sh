cargo build --release --target wasm32-unknown-unknown 
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rougelike.wasm 
mkdir output
cp index.html output
cp -r assets output
cp -r out output