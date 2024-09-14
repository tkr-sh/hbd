commands_to_test=(
    # Help
    " "
    "hbd --help"
    # Add user1
    "hbd remove user1"
    "hbd add user1 06-06" 
    # Add user2
    "hbd remove user2"
    "hbd add user2 06-06"
    # Add existing user
    " "
    "hbd add user2 06-07"
    # Rename user
    "hbd rename user user2"
    "hbd rename user2 user" 
    # List
    " "
    "hbd list" 
    # List limit
    " "
    "hbd list -l 100" 
    # List limit 
    " "
    "hbd list -L 100" 
    # Get
    " "
    "hbd get"
)

# Save the current storage
cp ~/.local/share/hbd/birthdays.json ~/.local/share/hbd/birthdays.json.bak
rm ~/.local/share/hbd/birthdays.json

cargo build --release



# Create test data
echo "Creating test data..."
./target/release/hbd import --check-duplicate false ./import
echo "Done!"


printf 'This is the metrics for 1 million birthdays registered.\n\n' > METRICS.md
for ((i=0; i<"${#commands_to_test[@]}"; i+=2)); do

    # Commands to execute
    before="${commands_to_test[$i]}"
    cmd="${commands_to_test[$(( i + 1 ))]}"
    
    echo "**Benchmark $(( i / 2 + 1 ))**: \"$cmd\"" >> METRICS.md


    cmd_to_exec_before="$(echo "$before" | sed 's,hbd,./target/release/hbd,g')"
    cmd_to_exec="$(echo "$cmd" | sed 's,hbd,./target/release/hbd,g')"

    echo "$cmd_to_exec"

    hyperfine -i --prepare "$cmd_to_exec_before" --warmup 5 "$cmd_to_exec" --export-markdown _METRICS$i.md

    # Format metrics
    cat _METRICS$i.md | sed "s/eval \"\$cmd_to_exec\"/$cmd/g" >> METRICS.md
    printf '\n' >> METRICS.md
    rm _METRICS$i.md
done

cp ~/.local/share/hbd/birthdays.json.bak ~/.local/share/hbd/birthdays.json
rm ~/.local/share/hbd/birthdays.json.bak
