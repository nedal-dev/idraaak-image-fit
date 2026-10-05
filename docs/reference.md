# API and errors

## Public types

| Type | Purpose | Fields or methods |
| --- | --- | --- |
| `Size` | Nonzero width and height | `Size::new(width, height)`, `width()`, `height()` |
| `CropRect` | Rectangle to keep in the resized image | Public `x`, `y`, `width`, `height` fields |
| `CoverPlan` | Resize operation followed by a crop | Public `resized: Size` and `crop: CropRect` fields |
| `FitError` | A reason an input or result cannot be used | Error variants described below |

All dimensions and coordinates are `u32`. The largest representable value is **4,294,967,295**. This is a numeric limit, not a promise that your image-processing library can allocate an image of that size.

`Size` keeps its fields private so zero dimensions cannot be introduced through normal construction. `CropRect` and `CoverPlan` have public fields; your own manually constructed plans remain your responsibility.

## Functions

```rust
pub fn contain(
    source: Size,
    bounds: Size,
    allow_upscale: bool,
) -> Result<Size, FitError>;

pub fn cover(
    source: Size,
    target: Size,
    allow_upscale: bool,
) -> Result<CoverPlan, FitError>;
```

These declarations summarize the API; they are not a standalone runnable program.

Contain uses the smaller scale required by the two bounds and rounds down. Cover uses the larger scale required by the two target dimensions and rounds up. Calculations use `u128` intermediates to avoid overflowing when comparing or multiplying `u32` inputs.

With enlargement disabled, contain returns a source that already fits unchanged. Cover fails if its required scale is greater than one.

## Error handling

| Variant | Cause | A useful response in the caller |
| --- | --- | --- |
| `InvalidDimensions { width, height }` | `Size::new` received a zero width or height | Ask for positive dimensions |
| `UnrepresentableSize` | Contain would round a dimension to zero | Choose larger bounds or decline the operation |
| `DimensionOverflow` | Cover's resized dimensions would exceed `u32::MAX` | Reduce the target or use a different layout |
| `UpscalingRequired` | Cover needs enlargement but `allow_upscale` is `false` | Keep the original size, choose a smaller target, or explicitly allow enlargement |

`FitError` implements `Display` and `std::error::Error`, so applications can show a readable error or propagate it with `?`.

Inputs are unsigned integers. Validate negative, non-integer, or out-of-range user input before converting it to `u32`; a cast is not input validation.

## Coordinate convention

The top-left pixel is **x = 0, y = 0**. A crop rectangle's width and height are sizes, not inclusive end coordinates. For example, `x = 31, width = 500` keeps the horizontal range from **31** up to, but not including, **531**.

Always resize first and crop second. Coordinates from `cover` do not refer to the original image.

## Scope and limitations

- Layout calculations do not choose an image format or JPEG quality.
- Centered cropping does not detect faces, text, or the most interesting subject.
- No rotation, EXIF orientation correction, focal-point selection, or image interpolation is performed.
- Integer rounding may slightly change the output aspect ratio.
- The library makes no file or network requests and performs no image-buffer allocation.

The full generated API documentation is available on [docs.rs](https://docs.rs/idraaak-image-fit/latest/idraaak_image_fit/). Report reproducible problems in the [project issue tracker](https://github.com/nedal-dev/idraaak-image-fit/issues).
