use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{self, Context};
use image;

#[derive(StructOpt, Debug)]
/// imot - image operating tool
/// 
/// Performs operations on a given image and writes output to stdout
/// if another path is not specified by -o or --output option.
/// In case of writing to stdout the output image format is derived from the source file extension.
/// 
/// EXAMPLE:
/// 
/// imot --contrast="-20" --flipv source.png > result.png
struct Opt {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Rotate image 90 degrees clockwise
    #[structopt(long = "rotate90")]
    rotate90: bool,
    /// Rotate image 180 degrees clockwise
    #[structopt(long = "rotate180")]
    rotate180: bool,
    /// Rotate image 270 degrees clockwise
    #[structopt(long = "rotate270")]
    rotate270: bool,
    /// Flip horizontally
    #[structopt(long = "fliph")]
    fliph: bool,
    /// Flip vertically
    #[structopt(long = "flipv")]
    flipv: bool,
    /// Adjust contrast. Negative values decrease the contrast
    /// and positive values increase the contrast
    #[structopt(long = "contrast")]
    contrast: Option<f32>,
    /// Performs a Gaussian blur on image.
    /// sigma is a measure of how much to blur by
    #[structopt(long = "blur", name = "sigma")]
    blur: Option<f32>,
    /// Adjust brightness. Negative values decrease the brightness
    /// and positive values increase the brightness
    #[structopt(long = "brightness")]
    brightness: Option<i32>,
    /// Set output filename
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let reader = image::io::Reader::open(&opt.path)
        .with_context(|| {
            format!("could not open file `{}`", &opt.path.display())
        })?;
    let format = reader.format().unwrap();
    let mut img = reader.decode()?;

    img = do_operations(img, &opt)?;

    if let Some(output) = opt.output {
        img.save(output)?;
    } else {
        img.write_to(&mut std::io::stdout(), format)?;
    }

    Ok(())
}

fn do_operations(img: image::DynamicImage, opt: &Opt) -> anyhow::Result<image::DynamicImage> {
    Some(img)
        .map(|img| {
            if opt.rotate90 {
                img.rotate90()
            } else {
                img
            }
        })
        .map(|img| {
            if opt.rotate180 {
                img.rotate180()
            } else {
                img
            }
        })
        .map(|img| {
            if opt.rotate270 {
                img.rotate270()
            } else {
                img
            }
        })
        .map(|img| {
            if opt.fliph {
                img.fliph()
            } else {
                img
            }
        })
        .map(|img| {
            if opt.flipv {
                img.flipv()
            } else {
                img
            }
        })
        .map(|img| {
            // FIXME: png images become transparent when using negative values
            opt.contrast.and_then(|c| Some(img.adjust_contrast(c)))
                .or(Some(img))
                .unwrap()
        })
        .map(|img| {
            opt.blur.and_then(|sigma| Some(img.blur(sigma)))
                .or(Some(img))
                .unwrap()
        })
        .map(|img| {
            opt.brightness.and_then(|b| Some(img.brighten(b)))
                .or(Some(img))
                .unwrap()
        })
        .ok_or(anyhow::anyhow!("something went wrong while doing operations"))
}