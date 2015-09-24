extern crate image;
use self::image::GenericImage;

pub fn crop(img: &mut self::image::DynamicImage) -> self::image::DynamicImage {
    img.crop(0, 0, 64, 64)
}

pub fn resize(img: &mut self::image::DynamicImage) -> self::image::DynamicImage {
    img.resize(32, 32, self::image::FilterType::Lanczos3)
}

pub fn trim_edges(img: &mut self::image::DynamicImage) {
    for i in (0..64) {
        (*img).put_pixel( i,  0, self::image::Rgba([0u8, 0u8, 0u8, 0u8]));
        (*img).put_pixel( i, 63, self::image::Rgba([0u8, 0u8, 0u8, 0u8]));
        (*img).put_pixel( 0,  i, self::image::Rgba([0u8, 0u8, 0u8, 0u8]));
        (*img).put_pixel(63,  i, self::image::Rgba([0u8, 0u8, 0u8, 0u8]));
    }
}
