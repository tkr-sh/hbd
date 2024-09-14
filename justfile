run:
    cargo run

cp-config:
    cp config.ron ~/.config/hbd/config.ron

metrics:
    just gen-import
    ./tests/metrics.sh

gen-import:
    cargo test -- --ignored
