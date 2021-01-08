# imot - image operating tool

Performs operations on a given image and writes output to stdout if another path is not specified by `-o` or `--output` option.
In case of writing to stdout the output image format is derived from the source file extension.

This tool is a yet anoter command line interface around the [image](https://crates.io/crates/image) crate.

### Supported operations
- rotate 90, 180 and 270 degrees clockwise
- flip horizontally and vertically
- adjust contrast and brightness
- blur

### Example usage
```
imot --contrast="-20" --flipv source.png > result.png
```

### Installation
If you really wanna use this tool, the recommended way to build it is using [cargo](https://doc.rust-lang.org/cargo/).

1. install cargo
2. clone this repo
3. in the root folder of this repo do `cargo build --release`
4. wait till it compiles ⏳
5. ready to use binary will be located at target/release/imot. Put it somewhere in PATH or use it by specifying a full path to this binary