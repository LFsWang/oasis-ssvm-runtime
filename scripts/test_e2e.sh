#!/bin/bash

set -euo pipefail

# For automatic cleanup on exit.
source ./scripts/common.sh

# Oasis core binaries.
oasis_node="${OASIS_CORE_ROOT_PATH}/go/oasis-node/oasis-node"
oasis_runner="${OASIS_CORE_ROOT_PATH}/go/oasis-net-runner/oasis-net-runner"

# Test client binary.
test_client="${RUNTIME_CARGO_TARGET_DIR}/debug/test-client"

# Prepare an empty data directory.
data_dir="/var/tmp/oasis-ssvm-runtime-runner"
rm -rf "${data_dir}"
mkdir -p "${data_dir}"
chmod -R go-rwx "${data_dir}"
client_socket="${data_dir}/net-runner/network/client-0/internal.sock"

# Run the network.
echo "Starting the test network."
${oasis_runner} \
    --fixture.file tests/fixture.json \
    --basedir.no_temp_dir \
    --basedir ${data_dir} &

# Wait for the validator node to be registered.
echo "Waiting for the validator node to be registered."
${oasis_node} debug control wait-nodes \
    --address unix:${client_socket} \
    --nodes 2 \
    --wait

# Advance epoch.
echo "Advancing epoch."
${oasis_node} debug control set-epoch \
    --address unix:${client_socket} \
    --epoch 1

# Wait for all nodes to be registered.
echo "Waiting for all nodes to be registered."
${oasis_node} debug control wait-nodes \
    --address unix:${client_socket} \
    --nodes 5 \
    --wait

# Advance epoch.
echo "Advancing epoch."
${oasis_node} debug control set-epoch \
    --address unix:${client_socket} \
    --epoch 2

# Start the test client.
echo "Starting the test client"
${test_client} \
    --node-address unix:${client_socket} \
    --runtime-id 8000000000000000000000000000000000000000000000000000000000000000
