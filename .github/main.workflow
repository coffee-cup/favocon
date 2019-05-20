workflow "Build and Test" {
  resolves = ["Cargo"]
  on = "push"
}

action "Cargo" {
  uses = "icepuma/rust-action@1.0.7"
  args = "cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test"
}
