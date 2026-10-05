use idraaak_image_fit::{contain, cover, FitError, Size};

fn main() -> Result<(), FitError> {
    let source = Size::new(1200, 800)?;
    let target = Size::new(500, 375)?;
    let contained = contain(source, target, false)?;
    let covered = cover(source, target, false)?;
    println!("Contain: {} x {}", contained.width(), contained.height());
    println!("Cover: {:?}; crop: {:?}", covered.resized, covered.crop);
    Ok(())
}
