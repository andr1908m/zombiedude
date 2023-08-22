#![no_std]
#![no_main]

use core::convert::Infallible;

use embedded_graphics::image::Image;
use embedded_graphics::mono_font::{ascii::FONT_6X12, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::Text;
use tinybmp::Bmp;

use psp::embedded_graphics::Framebuffer;

psp::module!("sample_emb_gfx", 1, 1);

fn psp_main() {
    psp::enable_home_button();
    
    render().unwrap();
}

fn render() -> Result<(), MyError> {
    let mut disp = Framebuffer::new();

    Rectangle::new(
        Point::new(0, 0), 
        Size::new(160, 80)
    ).into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb888::BLACK)
            .build()
    ).draw(&mut disp)?;

    let bmp = Bmp::from_slice(include_bytes!("../assets/ferris.bmp"))?;
    let ferris = Image::new(
        &bmp,
        Point::zero()
    );
    Image::draw(&ferris, &mut disp)?;

    Triangle::new(
        Point::new(8, 66 + 16),
        Point::new(8 + 16, 66 + 16),
        Point::new(8 + 8, 66),
    ).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .build(),
    ).draw(&mut disp)?;

    Rectangle::new(
        Point::new(36, 66), 
        Size::new(16, 16)
    ).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::GREEN)
            .stroke_width(1)
            .build(),
    ).draw(&mut disp)?;

    Circle::new(
        Point::new(72, 66 + 8), 
        8
    ).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::BLUE)
            .stroke_width(1)
            .build(),
    ).draw(&mut disp)?;

    Text::new(
        "Hello Rust!",
        Point::new(0, 86),
        MonoTextStyle::new(&FONT_6X12, Rgb888::new(255, 7, 0)),
    ).draw(&mut disp)?;
    
    Ok(())
}

#[derive(Debug)]
enum MyError {
    Infallible(Infallible),
    ParseError(tinybmp::ParseError)
}

impl From<tinybmp::ParseError> for MyError {
    fn from(value: tinybmp::ParseError) -> Self {
        MyError::ParseError(value)
    }
}

impl From<Infallible> for MyError {
    fn from(value: Infallible) -> Self {
        MyError::Infallible(value)
    }
}