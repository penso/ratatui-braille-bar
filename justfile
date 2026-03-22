# Run the dashboard example
demo:
    cargo run --example dashboard

# Record the demo GIF with vhs
record:
    vhs demo.tape

# Build in release mode
build:
    cargo build --release

# Publish a new release: just publish 0.3.0
publish version:
    sed -i '' 's/^version = ".*"/version = "{{version}}"/' Cargo.toml
    cargo generate-lockfile
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version to {{version}}"
    git push
    gh release create "v{{version}}" --title "v{{version}}" --generate-notes
