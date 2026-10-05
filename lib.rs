#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fmt;

/// Nonzero image dimensions in pixels, with each dimension fitting in `u32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    /// Creates a size, rejecting a zero width or height.
    ///
    /// # Errors
    /// Returns [`FitError::InvalidDimensions`] when either dimension is zero.
    pub const fn new(width: u32, height: u32) -> Result<Self, FitError> {
        if width == 0 || height == 0 {
            Err(FitError::InvalidDimensions { width, height })
        } else {
            Ok(Self { width, height })
        }
    }

    /// Returns the width in pixels.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Returns the height in pixels.
    pub const fn height(self) -> u32 {
        self.height
    }
}

/// A crop rectangle in the coordinates of the **resized** image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CropRect {
    /// Pixels to skip from the left edge.
    pub x: u32,
    /// Pixels to skip from the top edge.
    pub y: u32,
    /// Width to retain.
    pub width: u32,
    /// Height to retain.
    pub height: u32,
}

/// Resize dimensions followed by a centered crop to the requested target size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoverPlan {
    /// Resize the original image to this size first.
    pub resized: Size,
    /// Crop this rectangle from the resized image.
    pub crop: CropRect,
}

/// Reasons a requested image layout cannot be represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitError {
    /// The caller supplied a zero dimension.
    InvalidDimensions {
        /// The supplied width.
        width: u32,
        /// The supplied height.
        height: u32,
    },
    /// Rounding down would make a contain dimension zero pixels.
    UnrepresentableSize,
    /// A cover resize dimension would exceed `u32::MAX`.
    DimensionOverflow,
    /// Covering the target requires enlargement but enlargement was disabled.
    UpscalingRequired,
}

impl fmt::Display for FitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions { width, height } => write!(
                formatter,
                "width and height must be greater than zero (received {width} x {height})"
            ),
            Self::UnrepresentableSize => formatter.write_str(
                "the proportional contain size is smaller than one pixel in a dimension",
            ),
            Self::DimensionOverflow => {
                formatter.write_str("the resized image dimension exceeds u32::MAX")
            }
            Self::UpscalingRequired => {
                formatter.write_str("covering the target requires upscaling, which is disabled")
            }
        }
    }
}

impl std::error::Error for FitError {}

/// Fits the entire source inside `bounds`, without cropping.
///
/// A single proportional scale is used. The non-limiting dimension is rounded
/// **down** to an integer pixel so the result never exceeds the bounds. Integer
/// rounding can slightly change the final pixel ratio; no extra stretching is
/// applied. If `allow_upscale` is false, neither source dimension is enlarged.
///
/// # Errors
/// Returns [`FitError::UnrepresentableSize`] if proportional rounding would
/// produce a zero-pixel dimension. The function does not silently clamp to one
/// pixel, which could badly distort an extremely narrow image.
pub fn contain(source: Size, bounds: Size, allow_upscale: bool) -> Result<Size, FitError> {
    let width_limited = u128::from(source.width) * u128::from(bounds.height)
        >= u128::from(source.height) * u128::from(bounds.width);
    let (numerator, denominator) = if width_limited {
        (bounds.width, source.width)
    } else {
        (bounds.height, source.height)
    };

    if !allow_upscale && numerator >= denominator {
        return Ok(source);
    }

    let width = u128::from(source.width) * u128::from(numerator) / u128::from(denominator);
    let height = u128::from(source.height) * u128::from(numerator) / u128::from(denominator);
    if width == 0 || height == 0 {
        return Err(FitError::UnrepresentableSize);
    }
    // The limiting scale ensures both results are bounded by a u32 target.
    Ok(Size {
        width: width as u32,
        height: height as u32,
    })
}

/// Covers `target` completely, then supplies a centered crop rectangle.
///
/// A single proportional scale is used. The non-limiting dimension is rounded
/// **up** so no target pixel is left uncovered. For an odd number of removed
/// pixels, the right or bottom side loses the extra pixel. The crop coordinates
/// refer to the resized image, not the original image.
///
/// # Errors
/// Returns [`FitError::UpscalingRequired`] when enlargement is necessary and
/// `allow_upscale` is false. Returns [`FitError::DimensionOverflow`] when either
/// resized dimension cannot fit in `u32`. Arithmetic uses `u128` intermediates.
pub fn cover(source: Size, target: Size, allow_upscale: bool) -> Result<CoverPlan, FitError> {
    let width_limited = u128::from(target.width) * u128::from(source.height)
        >= u128::from(target.height) * u128::from(source.width);
    let (numerator, denominator) = if width_limited {
        (target.width, source.width)
    } else {
        (target.height, source.height)
    };

    if !allow_upscale && numerator > denominator {
        return Err(FitError::UpscalingRequired);
    }

    let ceil_scaled = |dimension: u32| {
        let product = u128::from(dimension) * u128::from(numerator);
        (product + u128::from(denominator) - 1) / u128::from(denominator)
    };
    let width = ceil_scaled(source.width);
    let height = ceil_scaled(source.height);
    if width > u128::from(u32::MAX) || height > u128::from(u32::MAX) {
        return Err(FitError::DimensionOverflow);
    }
    let resized = Size {
        width: width as u32,
        height: height as u32,
    };
    Ok(CoverPlan {
        resized,
        crop: CropRect {
            x: (resized.width - target.width) / 2,
            y: (resized.height - target.height) / 2,
            width: target.width,
            height: target.height,
        },
    })
}
