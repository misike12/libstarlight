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

## `.slpatch` format
`.slpatch` stands for Starlight Patch. The format is JSON-encoded data that is read by Serde.
Here is an example:
```json
{
    "name": "My really nice patch",
    "version": "1.2.3",
    "process": "Super_interesting_game.exe",
    "patches": [
        {
            "module": "secret_files.dll",
            "patterns": {
                "amd64": [
                    [
                        "DE (AD) (BE) EF",
                        "${1} DE EF ${2}"
                    ]
                ],
                "i386": [
                    [
                        "12 34 56 78",
                        "AB CD EF 09"
                    ]
                ]
            }
        }
    ]
}
```
- **`name`, `version`:** Standard metadata for patch file.
- **`process`:** The process name that the patches will apply to.
- **`patches`:** A list of patches that will be applied to the process. Each of these have a standard structure:
    - **`module`:** Module name that patches will be applied to.
    - **`patterns`**: Regex find/replace patterns for each architecture. Each of these have a standard structure:
        - **`(architecture): [pattern1, pattern2, ...]`**
        - **`architecture`** is a key. Possible values are `amd64`, `i386`, `arm` and `arm64`.
        - **`pattern(N)`** is a list consisting of two elements. First element is the pattern to be found, and second is the replace pattern. These should all be valid hex patterns.
