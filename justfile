env:
    source .env
dev:
    cargo run .
watch:
    cargo watch -c -w src -x run
