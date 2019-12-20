# Instant

If you call `std::time::Instant::now()` on a WASM platform, it will panic. This crate provides a partial
replacement for `std::time::Instant` that works on WASM too. This defines the type `instant::Instant` which is:

* A struct emulating the behavior of **std::time::Instant** if you are targeting `wasm32-unknown-unknown` or `wasm32-unknown-asmjs`
. This emulation is based on the javascript `performance.now()` function.
* A type alias for `std::time::Instant` otherwise.



Note that even if the **stdweb** or **wasm-bindgen** feature is enabled, this crate will continue to rely on `std::time::Instant`
as long as you are not targeting wasm32. This allows for portable code that will work on both native and WASM platforms.

### The function `now`.
the function `instant::now()` will be exported and will either:

* Call `performance.now()` when compiling for a WASM platform.
* Call `time::precise_time_s() * 1000.0` otherwise.

The result is expressed in milliseconds.

## Examples
_Cargo.toml_:
```toml
[dependencies]
instant = "0.1"
```

_main.rs_:
```rust
fn main() {
    // Will be the same as `std::time::Instant`.
    let now = instant::Instant::new();
}
```
