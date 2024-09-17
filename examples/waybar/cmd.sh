users="$(
    # Split on a random char
    cargo run -- list -s '|' |
    # Replace newlines by random char
    tr '\n' '&' |
    # Get the soonest day of birthday
    cut -d'|' -f1 |
    # Now we can get recover newlines
    tr '&' '\n' |
    # Remove the first one (In X days:)
    awk 'NR > 1' |
    # Trim whitespace
    sed 's/\s*$//g' |
    # Capture the name
    awk '{gsub(/Birthday of | !!/, "");gsub(/ \(Will be \w+ years old\)/,"");print}' |
    # Replace newlines by commas
    tr '\n' ',' |
    # Remove trailing commas
    sed 's/,*$//g'
)"


if [ -z "$(hbd get)" ]; then
    cargo run -- list | head -n 1 | sed 's/\x1b\[[0-9;]*m//g'  | awk "{printf \"%s %s $users\", \$2, \$3}"
else
    echo "Today: $(hbd get -s ',')"
fi
