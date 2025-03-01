use std::path::Path;

use image_convert::{
    identify_ping, to_bmp, to_gif, to_gray_raw, to_ico, to_jpg, to_pgm, to_png, to_tiff, to_webp,
    BMPConfig, GIFConfig, GrayRawConfig, ICOConfig, ImageResource, InterlaceType, JPGConfig,
    PGMConfig, PNGConfig, TIFFConfig, WEBPConfig,
};

const INPUT_IMAGE_PATH: &str = r"tests/data/P1060382.JPG";

#[test]
fn get_identify() {
    let input = ImageResource::from_path(INPUT_IMAGE_PATH);

    let id = identify_ping(&input).unwrap();

    assert_eq!(4592, id.resolution.width);
    assert_eq!(2584, id.resolution.height);
    assert_eq!("JPEG", id.format);
    assert_eq!((180.0f64, 180.0f64), id.ppi);
    assert_eq!(InterlaceType::NoInterlace, id.interlace);
}

#[test]
fn to_bmp_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.bmp");

    let mut config = BMPConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_bmp(&mut output, &input, &config).unwrap();
}

#[test]
fn to_jpg_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.jpg");

    let mut config = JPGConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_jpg(&mut output, &input, &config).unwrap();
}

#[test]
fn to_png_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.png");

    let mut config = PNGConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_png(&mut output, &input, &config).unwrap();
}

#[test]
fn to_gif_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.gif");

    let mut config = GIFConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_gif(&mut output, &input, &config).unwrap();
}

#[test]
fn to_tiff_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.tif");

    let mut config = TIFFConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_tiff(&mut output, &input, &config).unwrap();
}

#[test]
fn to_webp_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.webp");

    let mut config = WEBPConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_webp(&mut output, &input, &config).unwrap();
}

#[test]
fn to_ico_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.ico");

    let mut config = ICOConfig::new();

    config.size.push((256u16, 256u16));
    config.size.push((16u16, 16u16));
    config.size.push((128u16, 128u16));
    config.size.push((64u16, 64u16));
    config.size.push((32u16, 32u16));

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_ico(&mut output, &input, &config).unwrap();
}

#[test]
fn to_gray_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.raw");

    let mut config = GrayRawConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_gray_raw(&mut output, &input, &config).unwrap();
}

#[test]
fn to_pgm_file2file() {
    let source_image_path = Path::new(INPUT_IMAGE_PATH);

    let target_image_path = Path::join(source_image_path.parent().unwrap(), "P1060382_output.pgm");

    let mut config = PGMConfig::new();

    config.width = 1920;

    let input = ImageResource::from_path(source_image_path);

    let mut output = ImageResource::from_path(target_image_path);

    to_pgm(&mut output, &input, &config).unwrap();
}
