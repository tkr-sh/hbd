read -r -p "Are you sure that you want to deploy the new version (Make sure that you updated the version) ? (y/n): " answer

# Check the answer
if [[ "$answer" == "y" || "$answer" == "Y" ]]; then
    ./packages/arch.sh
fi
