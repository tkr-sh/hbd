run:
    cargo run

cp-config:
    cp config.ron ~/.config/hbd/config.ron

metrics:
    ./tests/metrics.sh

gen-import:
    cargo test gen_import
    # ./tests/create_import.sh
