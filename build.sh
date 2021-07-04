#!/bin/bash

# Build trust
cargo b --release
ext=$?
# Set the right capabilities for the Network Interface
bin_file="./target/release/trust"
sudo setcap cap_net_admin=eip $bin_file
# Exit if it didn't compile correctly:
if [[$ext -ne 0]] then
    exit $ext
fi
# Run trust on the Background
$CARGO_TARGET_DIR/release/trust &
# Get the PID of the process
pid=$!
# Set the Network Interface
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
# Make the script listen to trap signals
trap "kill $pid" TERM
# Wait for the background process to finish
wait $pid

