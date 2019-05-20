workflow "New workflow" {
  resolves = ["Rust Action"]
  on = "push"
}

action "Rust Action" {
  uses = "icepuma/rust-action@1.0.7"
  args = "cargo fmt -- --check && cargo clippy -- -Dwarnings && cargo test"
}
