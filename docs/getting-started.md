# Getting started

You need a Rust application with Cargo. The library supports Rust **1.70** or newer and has no third-party dependencies.

## Add the dependency

Add this entry to your application's `Cargo.toml`. If it already has a `[dependencies]` section, add only the package line beneath that existing heading.

```toml
[dependencies]
idraaak-image-fit = "0.1"
```

## Run a layout calculation

Put the following complete example in `src/main.rs` and run `cargo run` from your application's directory.

```rust
use idraaak_image_fit::{contain, cover, FitError, Size};

fn main() -> Result<(), FitError> {
    let source = Size::new(1200, 800)?;
    let box_size = Size::new(500, 375)?;

    let fitted = contain(source, box_size, false)?;
    println!("Contain: {} x {}", fitted.width(), fitted.height());

    let plan = cover(source, box_size, false)?;
    println!(
        "Cover resize: {} x {}",
        plan.resized.width(),
        plan.resized.height()
    );
    println!(
        "Cover crop: x={}, y={}, width={}, height={}",
        plan.crop.x, plan.crop.y, plan.crop.width, plan.crop.height
    );

    Ok(())
}
```

Expected output:

```text
Contain: 500 x 333
Cover resize: 563 x 375
Cover crop: x=31, y=0, width=500, height=375
```

`Size::new` validates dimensions. The `?` operator returns an error to the caller if a value is invalid or a layout cannot be represented. `main` returns `Result` so the example can use this error handling directly.

## Apply the result to an image

For contain, resize the source to the returned dimensions. If your final canvas must be exactly **500 x 375**, place the **500 x 333** result on that canvas and choose a background yourself. The library does not create the canvas or add padding.

For cover, perform these operations in order:

1. Resize the original image to **563 x 375**.
2. In that resized image, start at **x = 31, y = 0**.
3. Keep a rectangle of **500 x 375**.

The crop coordinates refer to the resized image. Applying them to the original **1200 x 800** image gives a different result.

## Decide whether enlargement is allowed

The final Boolean argument is `allow_upscale`:

- `false` keeps contain from enlarging an image that already fits. Cover returns `UpscalingRequired` if filling the target would require enlargement.
- `true` permits enlargement. It does not restore missing detail or guarantee that the enlarged image will look sharp.

See the [small-image recipe](recipes.md#keep-a-small-image-at-its-original-size) for concrete examples.
