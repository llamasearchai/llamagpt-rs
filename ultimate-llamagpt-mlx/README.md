# Ultimate LlamaGPT-MLX

Ultimate LlamaGPT-MLX is an advanced Rust-based CLI assistant that brings together the best of on-device ML inference and a rich terminal experience. Built for Apple Silicon and optimized for performance (including M3 Max Macs), it offers:
- Asynchronous function calling and smart prompt processing.
- An animated, interactive TUI powered by ratatui and crossterm.
- Persistent chat sessions with history.
- Docker containerization for easy deployment.
- Integrated support for MLX-Textgen, MLX-Embeddings, and MLX-VLM.
- Automated installation and configuration with OpenAI API key support.

## Installation

Run the provided setup script:
```bash
chmod +x setup_ultimate_llamagpt_mlx.sh
./setup_ultimate_llamagpt_mlx.sh
```

After running the script, follow the instructions printed at the end. Your OpenAI API key will be saved (via the OPENAI_API_KEY environment variable and in the generated configuration) so that the assistant can use it for API calls.

## Building and Running

Inside the project directory:
```bash
cd ultimate-llamagpt-mlx
cargo build --release
./target/release/llamagpt "Your one-shot prompt here"
```

For an interactive REPL:
```bash
./target/release/llamagpt
```

For Docker:
```bash
docker-compose build
docker-compose run llamagpt
```

## Configuration

Configuration is stored in \`~/.config/ultimate-llamagpt-mlx/config.toml\`. You can modify settings such as the theme, default model, and OpenAI API key there.

## Features

- **On-Device Inference:** Powered by MLX and optimized for Apple Silicon.
- **Animated TUI:** Enjoy a rich terminal UI with animations, themes, and advanced keybindings.
- **Chat Sessions:** Persistent conversations with history saving.
- **Extensible Commands:** Easily add new functions via the command registry.
- **Integrated MLX Modules:** Support for MLX-Textgen, MLX-Embeddings, and MLX-VLM for diverse ML tasks.

## License

This project is licensed under the MIT License.
