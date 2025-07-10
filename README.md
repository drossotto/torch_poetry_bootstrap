# torch_poetry_bootstrap

A simple command-line utility that detects your system's NVIDIA CUDA version and recommends the appropriate PyTorch wheel source. It outputs a `tool.poetry.source` TOML snippet for Poetry users, ensuring correct PyTorch installation with GPU support.

---

## Installation
If you have Rust installed, the CLI can be installed via cargo, like so:

```bash
cargo install torch_poetry_bootstrap
```

### 📦 Option 2: Download a Precompiled Binary

If you don’t want to install Rust, you can download a standalone binary from the [GitHub Releases](https://github.com/drossotto/torch_poetry_bootstrap/releases):

1. Visit the [Releases page](https://github.com/drossotto/torch_poetry_bootstrap/releases).
2. Download the binary appropriate for your system:
   - **Windows**: `torch_poetry_bootstrap.exe`
   - **Linux**: `torch_poetry_bootstrap`
   - **macOS**: `torch_poetry_bootstrap`
3. (Linux/macOS only) Make the binary executable:

   ```bash
   chmod +x torch_poetry_bootstrap


## Features

- Detects local CUDA version via `nvidia-smi`
- Resolves the most compatible PyTorch wheel index (e.g., `cu121`, `cu118`, etc.)
- Outputs a copy-pasteable TOML snippet for use in your `pyproject.toml`
- Supports dry-run and TOML print modes
- Logs messages to stdout and optionally to a file

---

## Example Usage

```bash
torch_poetry_bootstrap \
  --patch-pyproject examples/pyproject.toml \
  --output examples/patched.toml

🔍 Running `nvidia-smi` to detect CUDA version...
✅ Detected CUDA version: 12.3

📄 Loading torch source mapping from JSON...
✅ Selected best matching torch source:
🔗 Source: cu121
🌐 URL: https://download.pytorch.org/whl/cu121

✅ Successfully patched `pyproject.toml` with the new source: examples/patched.toml

```