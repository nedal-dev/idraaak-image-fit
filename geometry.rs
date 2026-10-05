use idraaak_image_fit::{contain, cover, CropRect, FitError, Size};

fn size(w: u32, h: u32) -> Size {
    Size::new(w, h).unwrap()
}

#[test]
fn rejects_zero_dimensions() {
    for (w, h) in [(0, 0), (0, 100), (100, 0)] {
        assert_eq!(
            Size::new(w, h),
            Err(FitError::InvalidDimensions {
                width: w,
                height: h
            })
        );
    }
}

#[test]
fn landscape_contain_and_cover() {
    let original = size(1200, 800);
    let target = size(500, 375);
    assert_eq!(contain(original, target, false), Ok(size(500, 333)));
    let plan = cover(original, target, false).unwrap();
    assert_eq!(plan.resized, size(563, 375));
    assert_eq!(
        plan.crop,
        CropRect {
            x: 31,
            y: 0,
            width: 500,
            height: 375
        }
    );
}

#[test]
fn portrait_contain_and_cover() {
    let original = size(800, 1200);
    let target = size(500, 375);
    assert_eq!(contain(original, target, false), Ok(size(250, 375)));
    let plan = cover(original, target, false).unwrap();
    assert_eq!(plan.resized, size(500, 750));
    assert_eq!(plan.crop.y, 187);
    assert_eq!(plan.resized.height() - plan.crop.y - plan.crop.height, 188);
}

#[test]
fn exact_ratio_and_identity() {
    assert_eq!(
        contain(size(1600, 900), size(800, 450), false),
        Ok(size(800, 450))
    );
    for source in [size(1, 1), size(123, 456), size(u32::MAX, u32::MAX)] {
        assert_eq!(contain(source, source, false), Ok(source));
        let plan = cover(source, source, false).unwrap();
        assert_eq!(plan.resized, source);
        assert_eq!((plan.crop.x, plan.crop.y), (0, 0));
    }
}

#[test]
fn prevents_enlargement_in_contain() {
    assert_eq!(
        contain(size(100, 50), size(500, 375), false),
        Ok(size(100, 50))
    );
    assert_eq!(
        contain(size(100, 50), size(500, 375), true),
        Ok(size(500, 250))
    );
    assert_eq!(
        contain(size(100, 1000), size(200, 100), false),
        Ok(size(10, 100))
    );
}

#[test]
fn cover_must_not_silently_leave_empty_pixels() {
    assert_eq!(
        cover(size(100, 50), size(500, 375), false),
        Err(FitError::UpscalingRequired)
    );
    assert_eq!(
        cover(size(1000, 100), size(200, 200), false),
        Err(FitError::UpscalingRequired)
    );
    assert_eq!(
        cover(size(100, 50), size(500, 375), true).unwrap().resized,
        size(750, 375)
    );
}

#[test]
fn rejects_zero_after_proportional_rounding() {
    assert_eq!(
        contain(size(1, u32::MAX), size(1, 1), false),
        Err(FitError::UnrepresentableSize)
    );
    assert_eq!(
        contain(size(u32::MAX, 1), size(1, 1), false),
        Err(FitError::UnrepresentableSize)
    );
}

#[test]
fn cover_overflow_is_checked() {
    assert_eq!(
        cover(size(u32::MAX, 1), size(1, u32::MAX), true),
        Err(FitError::DimensionOverflow)
    );
    assert_eq!(
        cover(size(1, u32::MAX), size(u32::MAX, 1), true),
        Err(FitError::DimensionOverflow)
    );
    assert_eq!(
        contain(
            size(u32::MAX, u32::MAX - 1),
            size(u32::MAX - 1, u32::MAX),
            true
        ),
        Ok(size(u32::MAX - 1, u32::MAX - 2))
    );
}

#[test]
fn boundaries_and_crop_centering_hold_over_a_grid() {
    let dimensions = [1, 2, 3, 7, 16, 99, 500, u32::MAX - 1, u32::MAX];
    for sw in dimensions {
        for sh in dimensions {
            for tw in dimensions {
                for th in dimensions {
                    let source = size(sw, sh);
                    let target = size(tw, th);
                    for allow in [false, true] {
                        if let Ok(output) = contain(source, target, allow) {
                            assert!(output.width() <= tw && output.height() <= th);
                            if !allow {
                                assert!(output.width() <= sw && output.height() <= sh);
                            }
                            // Each pixel dimension is less than one pixel from the exact scale.
                            let (num, den) = if u128::from(sw) * u128::from(th)
                                >= u128::from(sh) * u128::from(tw)
                            {
                                (tw, sw)
                            } else {
                                (th, sh)
                            };
                            if allow || num < den {
                                for (input, result) in [(sw, output.width()), (sh, output.height())]
                                {
                                    let exact = u128::from(input) * u128::from(num);
                                    let rounded = u128::from(result) * u128::from(den);
                                    assert!(rounded <= exact && exact - rounded < u128::from(den));
                                }
                            }
                        }
                        if let Ok(plan) = cover(source, target, allow) {
                            assert!(plan.resized.width() >= tw && plan.resized.height() >= th);
                            assert_eq!((plan.crop.width, plan.crop.height), (tw, th));
                            assert!(plan.crop.x <= plan.resized.width() - tw);
                            assert!(plan.crop.y <= plan.resized.height() - th);
                            assert!((plan.resized.width() - tw) - 2 * plan.crop.x <= 1);
                            assert!((plan.resized.height() - th) - 2 * plan.crop.y <= 1);
                            if !allow {
                                assert!(plan.resized.width() <= sw && plan.resized.height() <= sh);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn errors_have_explanatory_messages() {
    for error in [
        FitError::InvalidDimensions {
            width: 0,
            height: 1,
        },
        FitError::UnrepresentableSize,
        FitError::DimensionOverflow,
        FitError::UpscalingRequired,
    ] {
        assert!(!error.to_string().is_empty());
        let standard_error: &dyn std::error::Error = &error;
        assert!(standard_error.source().is_none());
    }
}
