# Idraaak Image Fit

A small, dependency-free Rust library for calculating integer image resize dimensions and centered crop rectangles. It computes geometry only: it does **not** read, resize, crop, compress, or write image files.

Created for [Idraaak](https://idraaak.com/). Released under the MIT license. Prepared with AI assistance; the implementation and examples are checked by automated tests.

## Install

```toml
[dependencies]
idraaak-image-fit = "0.1"
```

## Contain: keep the whole image visible

```rust
use idraaak_image_fit::{contain, Size};

let source = Size::new(1200, 800)?;
let bounds = Size::new(500, 375)?;
let output = contain(source, bounds, false)?;
assert_eq!((output.width(), output.height()), (500, 333));
# Ok::<(), idraaak_image_fit::FitError>(())
```

This fits the entire image inside the box. It does not add padding or remove pixels. A separate image-processing library can use the returned dimensions.

## Cover: fill the box and crop the excess

```rust
use idraaak_image_fit::{cover, Size};

let source = Size::new(1200, 800)?;
let target = Size::new(500, 375)?;
let plan = cover(source, target, false)?;
assert_eq!((plan.resized.width(), plan.resized.height()), (563, 375));
assert_eq!((plan.crop.x, plan.crop.y), (31, 0));
assert_eq!((plan.crop.width, plan.crop.height), (500, 375));
# Ok::<(), idraaak_image_fit::FitError>(())
```

First resize to **563 × 375**, then keep a **500 × 375** rectangle starting at **x = 31, y = 0** in the resized image. This removes 31 pixels from the left and 32 from the right. The coordinates are not original-image coordinates. The library provides the plan; your image-processing code performs the operations.

## Rounding and enlargement

- Both modes use one proportional scale. Pixel dimensions must be integers, so the final pixel ratio may differ slightly from the exact mathematical ratio.
- `contain` rounds the non-limiting dimension **down**, ensuring the image fits inside the bounds.
- `cover` rounds the non-limiting dimension **up**, ensuring the target is fully covered. Centered crops put an odd extra removed pixel on the right or bottom.
- Pass `false` to disable enlargement. Contain then returns the source unchanged when it already fits; cover returns `UpscalingRequired` if filling the target would require enlargement.
- Zero dimensions are rejected by `Size::new`. An extreme contain request that would round a dimension to zero returns `UnrepresentableSize` instead of silently stretching a one-pixel image.
- Inputs and output dimensions use `u32`. Calculations use `u128` intermediates. Cover results larger than `u32::MAX` return `DimensionOverflow`.
- Negative or non-integer measurements must be rejected by the calling program before conversion to `u32`.

## Errors

`FitError` implements `Display` and `std::error::Error`, with separate variants for invalid dimensions, a subpixel contain result, a cover overflow, and forbidden enlargement. The library does not allocate image buffers and does not perform network or file operations.

## Development

```text
cargo test --all-targets
cargo test --doc
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps
cargo publish --dry-run
```

Minimum supported Rust version: **1.70**, edition **2021**. No third-party dependencies.

