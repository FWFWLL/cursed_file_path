# cursed_file_path
Cursed file path concatenation
```rust
let path_operator_overload = Path::new_relative() / "Dev" / "Rust" / ".Experiments";

let path_builder_pattern = Path::new_relative()
    .append("Dev")
    .append("Rust")
    .append(".Experiments");
```
