extern crate image;

use image::{Rgb, RgbImage};
pub struct MyRgbImage {
    img: RgbImage,
}

#[allow(dead_code)]
impl MyRgbImage {
    pub fn new(a_image: RgbImage) -> Self {
        MyRgbImage { img: a_image }
    }

    // todo!("check image bounds")
    pub fn get_line(&self, line: u32) -> Vec<Rgb<u8>> {
        (0..self.img.height())
            .map(|i| self.img[(line, i)])
            .collect::<Vec<_>>()
    }

    // todo!("check image bounds")
    pub fn get_column(&self, column: u32) -> Vec<Rgb<u8>> {
        (0..self.img.width())
            .map(|i| self.img[(i, column)])
            .collect::<Vec<_>>()
    }

    pub fn get_columns_left_to_right(&self, last_column: u32) -> Vec<Rgb<u8>> {
        (0..last_column)
            .flat_map(|column| self.get_column(column))
            .collect::<Vec<_>>()
    }

    pub fn get_columns_right_to_left(&self, last_column: u32) -> Vec<Rgb<u8>> {
        (self.img.width()..last_column)
            .flat_map(|column| self.get_column(column))
            .collect::<Vec<_>>()
    }

    pub fn get_lines_top_down(&self, last_line: u32) -> Vec<Rgb<u8>> {
        (0..last_line)
            .flat_map(|line| self.get_line(line))
            .collect::<Vec<_>>()
    }

    pub fn get_lines_bottom_up(&self, last_line: u32) -> Vec<Rgb<u8>> {
        (self.img.height()..last_line)
            .flat_map(|line| self.get_line(line))
            .collect::<Vec<_>>()
    }

    pub fn blend_segment(
        &self,
        mut segment: Vec<Rgb<u8>>,
        blender: fn(&mut Rgb<u8>) -> Rgb<u8>,
    ) -> Vec<Rgb<u8>> {
        segment
            .iter_mut()
            .map(|pixel| blender(pixel))
            .collect::<Vec<_>>()
    }

    pub fn blend_line(&mut self, line: u32, pixel_line: Vec<Rgb<u8>>) {
        (0..self.img.height()).for_each(|y| self.img[(line, y)] = pixel_line[y as usize]);
    }

    pub fn blend_colum(&mut self, column: u32, pixel_column: Vec<Rgb<u8>>) {
        (0..self.img.width()).for_each(|x| self.img[(x, column)] = pixel_column[x as usize]);
    }

    pub fn swap_lines(&mut self, line1: u32, line2: u32) {
        let line_1 = self.get_line(line1);
        let line_2 = self.get_line(line2);
        self.blend_line(line1, line_2);
        self.blend_line(line2, line_1);
    }

    pub fn swap_columns(&mut self, column1: u32, column2: u32) {
        let column_1 = self.get_column(column1);
        let column_2 = self.get_column(column2);
        self.blend_colum(column1, column_2);
        self.blend_colum(column2, column_1);
    }

    pub fn mess_everything(&mut self) {
        (0..self.img.width() / 2)
            .zip(self.img.width() / 2..self.img.width())
            .for_each(|i| self.swap_lines(i.0, i.1));
        (0..self.img.height() / 2)
            .zip((self.img.height() / 2)..self.img.height())
            .for_each(|i| self.swap_columns(i.0, i.1));
    }

    pub fn save_image(&self, path: &str) {
        self.img.save(path).expect("algo n deu certo.");
    }
}

#[allow(dead_code)]
pub enum RgbFilter {
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
    SorteColors,
    SwapRgbColors,
}

#[allow(dead_code)]
pub fn apply_filter(filter: RgbFilter, pixel: &mut Rgb<u8>) -> Rgb<u8> {
    match filter {
        RgbFilter::Red => RgbFilter::red(pixel),
        RgbFilter::Green => RgbFilter::green(pixel),
        RgbFilter::Blue => RgbFilter::blue(pixel),
        RgbFilter::Magenta => RgbFilter::magenta(pixel),
        RgbFilter::Cyan => RgbFilter::cyan(pixel),
        RgbFilter::Yellow => RgbFilter::yellow(pixel),
        RgbFilter::SorteColors => RgbFilter::sorted_colors(pixel),
        RgbFilter::SwapRgbColors => RgbFilter::swap_rgb_colors(pixel),
    }
}

#[allow(dead_code)]
impl RgbFilter {
    pub fn blue(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], rgb[1], 255])
    }

    pub fn red(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([255, rgb[1], rgb[2]])
    }

    pub fn green(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], 255, rgb[2]])
    }

    pub fn magenta(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([255, rgb[1], 255])
    }

    pub fn cyan(rgb: &Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], 255, 255])
    }

    pub fn yellow(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([255, 255, rgb[2]])
    }

    pub fn sorted_colors(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        rgb.0.sort_unstable();
        Rgb([rgb[0], rgb[1], rgb[2]])
    }

    pub fn sorted_colors_rev(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        rgb.0.sort_unstable();
        rgb.0.reverse();
        Rgb([rgb[0], rgb[1], rgb[2]])
    }

    pub fn swap_rgb_colors(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[2], rgb[0], rgb[1]])
    }
}
