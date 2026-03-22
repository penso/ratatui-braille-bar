# Run the dashboard example
demo:
    cargo run --example dashboard

# Record the demo GIF with vhs
record:
    vhs demo.tape

# Build in release mode
build:
    cargo build --release
