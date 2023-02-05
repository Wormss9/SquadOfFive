export $(grep -v '^#' ../.env.2.local | xargs)
cargo run