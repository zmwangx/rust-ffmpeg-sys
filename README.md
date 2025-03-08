[![ffmpeg-sys-next on crates.io](https://img.shields.io/crates/v/ffmpeg-sys-next?cacheSeconds=3600)](https://crates.io/crates/ffmpeg-sys-next)
[![build](https://github.com/zmwangx/rust-ffmpeg-sys/workflows/build/badge.svg)](https://github.com/zmwangx/rust-ffmpeg-sys/actions)

This is a fork of the abandoned [ffmpeg-sys](https://github.com/meh/rust-ffmpeg-sys) crate. You can find this crate as [ffmpeg-sys-next](https://crates.io/crates/ffmpeg-sys-next) on crates.io.

This crate contains low level bindings to FFmpeg. You're probably interested in the high level bindings instead: [ffmpeg-next](https://github.com/zmwangx/rust-ffmpeg).

A word on versioning: major and minor versions track major and minor versions of FFmpeg, e.g. 4.2.x of this crate has been updated to support the 4.2.x series of FFmpeg. Patch level is reserved for bug fixes of this crate and does not track FFmpeg patch versions.

## Feature flags

In addition to feature flags declared in `Cargo.toml`, this crate performs various compile-time version and feature detections and exposes the results in additional flags. These flags are briefly documented below; run `cargo build -vv` to view more details.

- `ffmpeg_<x>_<y>` flags (new in v4.3.2), e.g. `ffmpeg_4_4`, indicating the FFmpeg installation being compiled against is at least version `<x>.<y>`. Currently available:

  - `ffmpeg_3_0`
  - `ffmpeg_3_1`
  - `ffmpeg_3_2`
  - `ffmpeg_3_3`
  - `ffmpeg_3_1`
  - `ffmpeg_4_0`
  - `ffmpeg_4_1`
  - `ffmpeg_4_2`
  - `ffmpeg_4_3`
  - `ffmpeg_4_4`

- `avcodec_version_greater_than_<x>_<y>` (new in v4.3.2), e.g., `avcodec_version_greater_than_58_90`. The name should be self-explanatory.

- `ff_api_<feature>`, e.g. `ff_api_vaapi`, corresponding to whether their respective uppercase deprecation guards evaluate to true.

- `ff_api_<feature>_is_defined`, e.g. `ff_api_vappi_is_defined`, similar to above except these are enabled as long as the corresponding deprecation guards are defined.

# Troubleshooting rust-ffmpeg-sys Build Error:

## Error Description

When compiling a Rust project that depends on `ffmpeg-sys-next`, you encounter:

```text
error: failed to run custom build command for `ffmpeg-sys-next`

--- stderr
thread 'main' panicked at build.rs:795:14:
called `Result::unwrap()` on an `Err` value:
pkg-config exited with status code 1
> PKG_CONFIG_ALLOW_SYSTEM_LIBS=1 pkg-config --libs --cflags libavutil

The system library `libavutil` required by crate `ffmpeg-sys-next` was not found.
```

### Causes:

1. **FFmpeg Not Installed**: The Rust crate `ffmpeg-sys-next` requires FFmpeg’s development files (headers and libraries).
2. **Missing `.pc` Files**: `pkg-config` (a tool to locate libraries) cannot find `libavutil.pc`, which describes how to link to FFmpeg.
3. **Environment Variables Not Configured**: `PKG_CONFIG_PATH` or `VCPKG_ROOT` is not set, so the build system cannot locate FFmpeg.

---

## Install FFmpeg via `vcpkg` (Recommended for Windows)

### Step 1: Install `vcpkg`

1. Clone the `vcpkg` repository:
   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git
   cd vcpkg
   ```
2. Bootstrap `vcpkg`:
   ```powershell
   .\bootstrap-vcpkg.bat
   ```

### Step 2: Install FFmpeg Libraries

Install FFmpeg for static linking (64-bit Windows):

```powershell
.\vcpkg install ffmpeg:x64-windows-static
```

### Step 3: Integrate `vcpkg` with Your System

```powershell
.\vcpkg integrate install
```

This configures your system to automatically find libraries installed via `vcpkg`.

### Step 4: Set Environment Variables

1. Set `VCPKG_ROOT` to your `vcpkg` directory (replace `C:\vcpkg` with your actual path):
   ```powershell
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```
2. Restart your terminal/PowerShell to apply changes.

### Step 5: Rebuild Your Project

```powershell
cargo clean && cargo build
```
