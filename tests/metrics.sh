user1="$(shuf -er -n8  {A..Z} {a..z} {0..9} | paste -sd "")"
user2="$(shuf -er -n8  {A..Z} {a..z} {0..9} | paste -sd "")"

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
    # Remove user
    "hbd add user1 06-06"
    "hbd remove user1"
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
for ((i=1; i<=100000; i++)); do
    random_month=$( printf "%02d" $((RANDOM % 12 + 1)) )
    random_day=$( printf "%02d" $((RANDOM % 28 + 1)) )
    user="$(shuf -er -n16  {A..Z} {a..z} {0..9} | paste -sd "")"
    ./target/release/hbd add "$user" "$random_month-$random_day"
    echo "$i/100 000 ..."
done
echo "Done!"


echo '' > METRICS.md
for ((i=0; i<"${#commands_to_test[@]}"; i+=2)); do

    # Commands to execute
    before="${commands_to_test[$i]}"
    cmd="${commands_to_test[$(( i + 1 ))]}"
    
    echo "**Benchmark $(( i / 2 + 1 ))**: \"$cmd\"" >> METRICS.md


    cmd_to_exec_before="$(echo "$before" | sed 's,hbd,./target/release/hbd,g')"
    cmd_to_exec="$(echo "$cmd" | sed 's,hbd,./target/release/hbd,g')"
    hyperfine --prepare 'eval "$cmd_to_exec_before"' --warmup 5 'eval "$cmd_to_exec"' --export-markdown METRICS$i.md

    # Format metrics
    cat METRICS$i.md | sed "s/eval \"\$cmd_to_exec\"/$cmd/g" >> METRICS.md
    printf '\n' >> METRICS.md
    rm METRICS$i.md
done

cp ~/.local/share/hbd/birthdays.json.bak ~/.local/share/hbd/birthdays.json
