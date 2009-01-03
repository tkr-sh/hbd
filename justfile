run:
    cargo run

cp-config:
    cp config.ron ~/.config/hbd/config.ron

metrics:
    just gen-import
    ./tests/metrics.sh

gen-import:
    cargo test -- --ignored

tape:
    save_date="$(date)"
    sudo date -s "2024-12-20 00:00"
    mv ~/.local/share/hbd/birthdays.json ~/.local/share/hbd/birthdays.json.bak || true
    vhs docs/demo.tape
    mv ~/.local/share/hbd/birthdays.json.bak ~/.local/share/hbd/birthdays.json
