<div align=center>
    <h1>libstarlight</h1>
    <p>Library for memory patching processes in Windows</p>
</div>

## Features
- Blazing fast, written in Rust
- Uses official Microsoft libraries for manipulating memory
- `.slpatch` format support
- Safe variable handling

## Usage
You can add it under `[dependencies]` in your `Cargo.toml` file like so:
```toml
[dependencies]
libstarlight = { git = "https://github.com/wavEye-Project/libstarlight.git" }
```
And import like so:
```rust
use libstarlight::slpatch;
use libstarlight::processhandle;
```
