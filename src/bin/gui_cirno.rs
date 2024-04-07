#![no_std]
#![no_main]

extern crate cirno_user;
use tinybmp::Bmp;
use embedded_graphics::{iterator::pixel, pixelcolor::Rgb888, draw_target::DrawTarget};
use embedded_graphics::prelude::Size;
use cirno_user::{exec, fork, waitpid, Display, VIRTGPU_XRES, VIRTGPU_YRES};
static BMP_DATA: &[u8] = include_bytes!("../../../img/cirno_resize_to_fit.bmp");
#[no_mangle]
pub fn main() -> i32 {
    let mut disp = Display::new(Size::new(VIRTGPU_XRES, VIRTGPU_YRES));
    let bmp = Bmp::<Rgb888>::from_slice(BMP_DATA).unwrap();
    let pixels = bmp.pixels();
    disp.draw_iter(pixels).unwrap();
    0
}
