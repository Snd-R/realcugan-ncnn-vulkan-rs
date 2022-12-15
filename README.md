## Building

required dependencies:

- cmake
- g++
- vulkan loader library
- ncnn
- glslang

1. run `git submodule update --init --recursive` to download subprojects required for build
2. set GLSLANG_TARGET_DIR environment variable (ubuntu: `/usr/lib/x86_64-linux-gnu/cmake/` arch linux: `/usr/lib/cmake`)
3. run `GLSLANG_TARGET_DIR=/usr/lib/cmake cargo build --release`

## Usage

```rust
use realcugan_ncnn_vulkan_rs::RealCugan;

fn main() {
    let image = image::open("image.png")?;

    let realcugan = RealCugan::new(
        config.gpuid,
        config.noise,
        config.scale,
        config.model,
        config.tile_size,
        config.sync_gap,
        config.tta_mode,
        config.num_threads,
        config.models_path,
    );

    realcugan.proc_image(image).save("output.png");
}
```