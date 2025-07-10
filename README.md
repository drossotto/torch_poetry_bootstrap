# torch_poetry_bootstrap

A simple command-line utility that detects your system's NVIDIA CUDA version and recommends the appropriate PyTorch wheel source. It outputs a `tool.poetry.source` TOML snippet for Poetry users, ensuring correct PyTorch installation with GPU support.

---

## Features

- Detects local CUDA version via `nvidia-smi`
- Resolves the most compatible PyTorch wheel index (e.g., `cu121`, `cu118`, etc.)
- Outputs a copy-pasteable TOML snippet for use in your `pyproject.toml`
- Supports dry-run and TOML print modes
- Logs messages to stdout and optionally to a file

---

## Example Usage

```bash
cargo run -- --print-toml
