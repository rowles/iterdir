# iterdir
Mini Rust Library for Filesystem Directory Iteration

```rust
let p = Path::new(".");
let pb = p.to_path_buf();

for fn in IterDir::new(&pb) {
  // do something with file
}
```
 [Cargo Crate](https://crates.io/crates/iterdir)
