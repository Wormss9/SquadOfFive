export $(grep -v '^#' ../.env.local | xargs)
cargo run