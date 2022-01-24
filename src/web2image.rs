use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::{fmt::Display, thread};

use anyhow::{anyhow, Result};
use headless_chrome::{
    protocol::page::{ScreenshotFormat, Viewport},
    Browser, LaunchOptionsBuilder,
};
use image::imageops::overlay;
use image::{load_from_memory, DynamicImage, ImageFormat, Luma, GenericImageView};
use qrcode::QrCode;

fn to_anyhow(e: impl Display) -> anyhow::Error {
    anyhow!(e.to_string())
}

pub fn url2image(url: &str) -> Result<DynamicImage> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .window_size(Some((1920, 1080)))
            .build()
            .unwrap(),
    )
    .map_err(to_anyhow)?;
    let tab = browser.wait_for_initial_tab().map_err(to_anyhow)?;
    let tab = tab
        .navigate_to(url)
        .map_err(to_anyhow)?
        .wait_until_navigated()
        .map_err(to_anyhow)?;

    let png_data = tab
        .capture_screenshot(
            ScreenshotFormat::PNG,
            Some(Viewport {
                x: 0 as f64,
                y: 0 as f64,
                width: 1920 as f64,
                height: 1080 as f64,
                scale: 1 as f64,
            }),
            true,
        )
        .map_err(to_anyhow)?;

    let img = load_from_memory(&png_data)?;

    Ok(img)
}

fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let code = QrCode::new(url.as_bytes())?;
    let imgBuff = code.render::<Luma<u8>>().build();
    let img = DynamicImage::ImageLuma8(imgBuff);
    Ok(img)
}

fn do_overlay(bottom: &mut DynamicImage, top: &DynamicImage) {
    let x = bottom.width() - top.width();
    let y = bottom.height() - top.height();
    overlay(bottom, top, x, y)
}

fn get_image_format(url: &str) -> Option<ImageFormat> {
    let path = Path::new(url);
    let ext = path
        .extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|s| {
            let ext_str = s.to_lowercase();
            match ext_str.as_str() {
                "jpg" => Some(ImageFormat::Jpeg),
                "png" => Some(ImageFormat::Png),
                _ => Some(ImageFormat::Png),
            }
        });

    return ext;
}

pub fn web2image(url: &str, output: &str) -> Result<()> {
    let mut bottom = url2image(url)?;
    let top = gen_qrcode(url)?;

    println!("开始合并");
    do_overlay(&mut bottom, &top);

    println!("开始保存");
    bottom.save_with_format(Path::new(output), get_image_format(output).unwrap());

    Ok(())
}
