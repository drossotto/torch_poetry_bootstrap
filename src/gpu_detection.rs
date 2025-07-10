// src/gpu_detection.rs

use std::fs;
use std::path::Path;
use std::process::Command;
use regex::Regex;

use crate::logs::{
    FAILED_NVIDIA_SMI_RUN
};

/// Attempts to detect CUDA version using `nvidia-smi`
pub fn detect_with_nvidia_smi() -> Result<String, String> {
    let output = Command::new("nvidia-smi")
        .output()
        .map_err(|_| FAILED_NVIDIA_SMI_RUN.to_string())?;

    let stdout = String::from_utf8(output.stdout)
        .map_err(|_| "Failed to parse nvidia-smi output as UTF-8".to_string())?;

    let re = Regex::new(r"CUDA Version: (\d+\.\d+)")
        .map_err(|_| "Failed to compile CUDA version regex".to_string())?;

    let captures = re.captures(&stdout)
        .ok_or("Failed to find CUDA version in nvidia-smi output".to_string())?;

    let version = captures.get(1)
        .ok_or("Failed to extract CUDA version group".to_string())?
        .as_str()
        .to_string();

    Ok(version)
}

/// Fallback method: look in version.txt or use `nvcc --version`
pub fn fallback_detect_cuda_version() -> Option<String> {
    let version_txt = Path::new("/usr/local/cuda/version.txt");
    if version_txt.exists() {
        if let Ok(content) = fs::read_to_string(version_txt) {
            for line in content.lines() {
                if line.starts_with("CUDA Version") {
                    return line.split_whitespace().nth(2).map(|s| s.to_string());
                }
            }
        }
    }

    if let Ok(output) = Command::new("nvcc").arg("--version").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(part) = line.split("release").nth(1) {
                return part.trim().split(',').next().map(|s| s.trim().to_string());
            }
        }
    }

    None
}