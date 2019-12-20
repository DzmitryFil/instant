extern crate wasm_bindgen_test;

use instant::{Instant, now};
use std::time::Duration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
// run these tests using: wasm-pack test --firefox --headless

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_instant_now() {
    let now1 = Instant::now();

    let mut sum = 0.0;
    for _ in 0..10000 {
        sum += now();
    }
    assert!(sum > 0.0);

    assert!(now1.elapsed().as_nanos() > 0);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_duration() {
    let now = Instant::now();
    let one_sec = Duration::from_secs(1);
    assert!(now.elapsed() < one_sec);
}
