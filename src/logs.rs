// src/logs.rs

pub const START_DETECTION: &str =
    "🔍 Running `nvidia-smi` to detect CUDA version...";

pub const CUDA_NOT_FOUND: &str =
    "❌ Could not detect CUDA version from `nvidia-smi` output.";

pub const FAILED_NVIDIA_SMI_RUN: &str =
    "❌ Unable to execute `nvidia-smi`. Please ensure that NVIDIA drivers are installed and that `nvidia-smi` is available in your system PATH.";

pub const LOADING_SOURCE_JSON: &str =
    "📄 Loading torch source mapping from JSON...";

pub const SOURCE_SELECTED: &str =
    "✅ Selected best matching torch source:";

pub const SELECTED_SOURCE_NAME: &str =
    "🔗 Source:";

pub const SELECTED_SOURCE_URL: &str =
    "🌐 URL:";

pub const PRINTED_TOML: &str = 
    "📦 TOML snippet for Poetry:";