#!/usr/bin/env bash
#
# Configure and run neb-chain nodes locally

# Log level
export RUST_LOG=info

# Number of nodes to run, fixed at 4 for now
node_count=4

# Nodes start listening for peer connections on this port
# Each node increments this value by 1
start_peer_port=7000

# Nodes start listening for public connections on this port
# Each node increments this value by 1
start_public_port=8000

# Save configs and dbs in the example dir
mkdir -p example
cd example

# If configs haven't been created yet, generate them
if [ ! -f node_1_cfg.toml ]; then

  # Generate the template config for $node_count validator nodes
  # Adds a backend public key for testing
  cargo run -- generate-template common.toml --validators-count $node_count --backend-key 8970eee89a78fae63e32ced013d07e248ae10cd1c5726f706b1559677d771d99

  # Generate public and secret keys for each node
  for i in $(seq 0 $((node_count - 1)))
  do
    peer_port=$((start_peer_port + i))
    cargo run -- generate-config common.toml pub_$((i + 1)).toml sec_$((i + 1)).toml --peer-address 127.0.0.1:${peer_port}
  done

  # Finalize configs for each node
  for i in $(seq 0 $((node_count - 1)))
  do
    public_port=$((start_public_port + i))
    private_port=$((public_port + node_count))
    cargo run -- finalize --public-api-address 0.0.0.0:${public_port} --private-api-address 0.0.0.0:${private_port} sec_$((i + 1)).toml node_$((i + 1))_cfg.toml --public-configs pub_*.toml
  done

fi

# Run all nodes
for i in $(seq 0 $((node_count - 1)))
do
  public_port=$((start_public_port + i))
  private_port=$((public_port + node_count))
  cargo run -- run --node-config node_$((i + 1))_cfg.toml --db-path db$((i + 1)) &
  echo "new node with ports: $public_port (public) and $private_port (private)"
  sleep 1
done


# If this script recieves a SIGINT (Ctrl-C), kill all child processes.
trap "pkill -P $$" SIGINT

wait
