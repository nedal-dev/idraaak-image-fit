# Image layout recipes

These examples calculate geometry for library release **0.1.0**. They do not perform image processing. Each Rust block is a complete program that can be placed in `src/main.rs` after adding the dependency shown in [getting started](getting-started.md).

## Keep a landscape image inside the bounds

Use contain when cutting off the image's edges would remove useful information, such as a screenshot label.

```rust
use idraaak_image_fit::{contain, FitError, Size};

fn main() -> Result<(), FitError> {
    let output = contain(Size::new(1200, 800)?, Size::new(500, 375)?, false)?;
    assert_eq!((output.width(), output.height()), (500, 333));
    Ok(())
}
```

The limiting dimension is width. The ideal height is **333.333...** pixels; contain rounds it down to **333** to stay within the bounds. The image is not cropped. A fixed-height container has **42** unused vertical pixels in total.

## Fill the same box with a landscape image

Use cover when a uniform thumbnail size matters more than retaining every edge.

```rust
use idraaak_image_fit::{cover, FitError, Size};

fn main() -> Result<(), FitError> {
    let plan = cover(Size::new(1200, 800)?, Size::new(500, 375)?, false)?;
    assert_eq!((plan.resized.width(), plan.resized.height()), (563, 375));
    assert_eq!((plan.crop.x, plan.crop.y), (31, 0));
    assert_eq!((plan.crop.width, plan.crop.height), (500, 375));
    Ok(())
}
```

Height sets the scale. The ideal resize width is **562.5**; cover rounds it up to **563** so the box is fully covered. The difference of **63** pixels is split as **31** on the left and **32** on the right. If an odd number of pixels must be removed, the right or bottom edge loses the extra one.

## Fit a portrait image

A **900 x 1200** portrait image gives a different tradeoff in the same **500 x 375** box.

```rust
use idraaak_image_fit::{contain, cover, FitError, Size};

fn main() -> Result<(), FitError> {
    let source = Size::new(900, 1200)?;
    let target = Size::new(500, 375)?;
    let fitted = contain(source, target, false)?;
    let plan = cover(source, target, false)?;

    assert_eq!((fitted.width(), fitted.height()), (281, 375));
    assert_eq!((plan.resized.width(), plan.resized.height()), (500, 667));
    assert_eq!((plan.crop.x, plan.crop.y), (0, 146));
    assert_eq!((plan.crop.width, plan.crop.height), (500, 375));
    Ok(())
}
```

Contain retains the full portrait but leaves horizontal space. Cover removes **146** pixels from the top and **146** from the bottom of the resized portrait. This is a centered crop, not face detection or subject-aware cropping.

## Keep a small image at its original size

When a **320 x 240** source is placed inside **800 x 600** bounds with enlargement disabled, contain returns **320 x 240**. Cover cannot fill those bounds without enlargement.

```rust
use idraaak_image_fit::{contain, cover, FitError, Size};

fn main() -> Result<(), FitError> {
    let source = Size::new(320, 240)?;
    let target = Size::new(800, 600)?;
    assert_eq!(contain(source, target, false)?, source);
    assert_eq!(cover(source, target, false), Err(FitError::UpscalingRequired));

    let enlarged = cover(source, target, true)?;
    assert_eq!((enlarged.resized.width(), enlarged.resized.height()), (800, 600));
    assert_eq!((enlarged.crop.x, enlarged.crop.y), (0, 0));
    Ok(())
}
```

Choose `true` only when your application permits enlargement. The geometry calculation cannot predict visual quality.

## Reject invalid or unrepresentable requests

Do not replace errors with invented dimensions. A zero dimension is invalid, and an extremely narrow image can round below one pixel in contain mode.

```rust
use idraaak_image_fit::{contain, FitError, Size};

fn main() -> Result<(), FitError> {
    assert_eq!(
        Size::new(0, 100),
        Err(FitError::InvalidDimensions { width: 0, height: 100 })
    );
    let result = contain(Size::new(1, 4000)?, Size::new(1, 1)?, false);
    assert_eq!(result, Err(FitError::UnrepresentableSize));
    Ok(())
}
```

Your calling code can choose a larger output, decline the operation, or apply another explicit fallback. The library avoids silently turning a subpixel result into a stretched one-pixel image.
