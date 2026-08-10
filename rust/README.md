# Rust path tracer

This is a small ray-tracing studio written from scratch in Rust. The command-line version renders a large PNG, while the same rendering library powers the live browser demo.

## Try it

From this directory, run:

```bash
cargo run --release -p tracer-cli -- --width 1600 --height 900 --samples 64 --output ../images/rust-tracer-render.png
```

The CLI prints rays per second as a rough local performance signal. It varies with
the scene, machine, and how many bounces each ray takes; it is not a benchmark.

The browser build is already included as `../rust-demo.wasm`. To rebuild it:

```bash
rustup target add wasm32-unknown-unknown
cargo build --release -p tracer-wasm --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/tracer_wasm.wasm ../rust-demo.wasm
```

## How a path tracer works

For every pixel, the camera sends a ray into the scene. When it hits an object, the tracer chooses a physically-inspired bounce: diffuse surfaces scatter light, metal reflects it, and glass either refracts or reflects it. It repeats this for several bounces and averages many slightly different rays. The result gradually converges from noisy dots into a soft, naturally lit image.

The core deliberately uses trait objects for scene objects: adding a new shape only requires implementing `Hittable`, without changing the renderer's dispatch logic. A tiny deterministic xorshift generator makes every render reproducible.
