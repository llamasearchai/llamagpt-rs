#!/bin/bash
# Quick launch script for Ultimate LlamaGPT-MLX
export RUST_LOG="${RUST_LOG:-info}"
if [[ "$(uname)" == "Darwin" && "$(uname -m)" == "arm64" ]]; then
  echo "🦙 Detected Apple Silicon Mac, enabling optimizations"
  export ULTIMATE_LLAMAGPT_USE_METAL=1
fi
if [ -f "./target/release/llamagpt" ]; then
  ./target/release/llamagpt "$@"
else
  echo "🦙 Building Ultimate LlamaGPT-MLX..."
  cargo build --release && ./target/release/llamagpt "$@"
fi
