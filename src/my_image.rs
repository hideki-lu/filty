use crate::error::{Error, Result};
use image::{Rgb, RgbImage};
use std::path::Path;

pub struct MyRgbImage {
    img: RgbImage,
}

#[allow(dead_code)]
impl MyRgbImage {
    pub fn new(a_image: RgbImage) -> Self {
        MyRgbImage { img: a_image }
    }

    pub fn get_line(&self, line: u32) -> Result<Vec<Rgb<u8>>> {
        if line >= self.img.width() {
            return Err(Error::IndexOutOfBounds);
        }

        Ok((0..self.img.height())
            .map(|i| self.img[(i, line)])
            .collect())
    }

    pub fn get_column(&self, column: u32) -> Result<Vec<Rgb<u8>>> {
        if column >= self.img.height() {
            return Err(Error::IndexOutOfBounds);
        }

        Ok((0..self.img.width())
            .map(|i| self.img[(column, i)])
            .collect())
    }

    pub fn get_columns_left_to_right(&self, last_column: u32) -> Vec<Rgb<u8>> {
        self.get_columns_interval(0, last_column)
    }

    pub fn get_columns_right_to_left(&self, last_column: u32) -> Vec<Rgb<u8>> {
        self.get_columns_interval(last_column, self.img.width())
            .into_iter()
            .rev()
            .collect()
    }

    pub fn get_lines_top_down(&self, last_line: u32) -> Vec<Rgb<u8>> {
        self.get_lines_interval(0, last_line)
    }

    pub fn get_lines_bottom_up(&self, last_line: u32) -> Vec<Rgb<u8>> {
        self.get_lines_interval(last_line, self.img.width())
            .into_iter()
            .rev()
            .collect()
    }

    pub fn get_lines_interval(&self, first_line: u32, last_line: u32) -> Vec<Rgb<u8>> {
        (first_line..=last_line)
            .map(|index| self.get_line(index))
            .filter_map(|line| line.ok())
            .flatten()
            .collect()
    }

    pub fn get_columns_interval(&self, first_column: u32, last_column: u32) -> Vec<Rgb<u8>> {
        (first_column..=last_column)
            .map(|index| self.get_column(index))
            .filter_map(|column| column.ok())
            .flatten()
            .collect()
    }

    pub fn blend_segment(&self, mut segment: Vec<Rgb<u8>>, blender: RgbFilter) -> Vec<Rgb<u8>> {
        segment
            .iter_mut()
            .map(|pixel| apply_filter(&blender, pixel))
            .collect()
    }

    pub fn blend_line(&mut self, line: u32, mut pixel_line: Vec<Rgb<u8>>) {
        if line < self.img.width() {
            (0..self.img.height()).for_each(|y| self.img[(line, y)] = pixel_line.pop().unwrap());
        } else {
            panic!(
                "index out of bounds, width is {}, got {}.",
                self.img.width(),
                line
            )
        }
    }

    pub fn blend_column(&mut self, column: u32, mut pixel_column: Vec<Rgb<u8>>) {
        if column < self.img.height() {
            (0..self.img.width()).for_each(|x| self.img[(x, column)] = pixel_column.pop().unwrap());
        } else {
            panic!(
                "index out of bounds, height is {}, got {}.",
                self.img.height(),
                column
            )
        }
    }

    pub fn blend_columns_interval(
        &mut self,
        first_column: u32,
        last_column: u32,
        columns: Vec<Rgb<u8>>,
    ) {
        let mut pieces = columns
            .chunks(self.img.width() as usize)
            .rev()
            .collect::<Vec<_>>();
        (first_column..last_column)
            .for_each(|column| self.blend_column(column, pieces.pop().unwrap().to_vec()));
    }

    pub fn blend_lines_interval(&mut self, first_line: u32, last_line: u32, lines: Vec<Rgb<u8>>) {
        let mut pieces = lines
            .chunks(self.img.height() as usize)
            .rev()
            .collect::<Vec<_>>();
        (first_line..last_line)
            .for_each(|line| self.blend_line(line, pieces.pop().unwrap().to_vec()));
    }

    pub fn swap_lines(&mut self, line1: u32, line2: u32) -> Result<()> {
        let line_1 = self.get_line(line1)?;
        let line_2 = self.get_line(line2)?;
        self.blend_line(line1, line_2);
        self.blend_line(line2, line_1);

        Ok(())
    }

    pub fn swap_columns(&mut self, column1: u32, column2: u32) -> Result<()> {
        let column_1 = self.get_column(column1)?;
        let column_2 = self.get_column(column2)?;
        self.blend_column(column1, column_2);
        self.blend_column(column2, column_1);

        Ok(())
    }

    pub fn mess_everything(&mut self) {
        (0..self.img.width())
            .filter(|i| *i % 4 == 0)
            .zip((0..self.img.width()).filter(|i| *i % 4 == 3))
            .into_iter()
            .for_each(|(odd, even)| self.swap_columns(odd, even).expect("oh lascou"));
        let a = self
            .get_lines_top_down(self.img.height())
            .into_iter()
            .rev()
            .collect();
        let b = self.blend_segment(a, RgbFilter::RgbOrMask(Rgb([40, 10, 200])));
        let c = self.blend_segment(b, RgbFilter::RgbAndMask(Rgb([0, 200, 250])));
        self.blend_lines_interval(0, self.img.width(), c);
    }

    pub fn save_image<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Ok(self.img.save(path)?)
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
    SortedColorsRev,
    SwapRgbColorsI,
    SwapRgbColorsII,
    SwapRgbColorsIII,
    RgbNot,
    RgbShlOnce,
    RgbShrOnce,
    RgbShlNth(u8),
    RgbShrNth(u8),
    RgbAndMask(Rgb<u8>),
    RgbOrMask(Rgb<u8>),
    RgbXorMask(Rgb<u8>),
}

#[allow(dead_code)]
pub fn apply_filter(filter: &RgbFilter, pixel: &mut Rgb<u8>) -> Rgb<u8> {
    match filter {
        RgbFilter::Red => RgbFilter::red(pixel),
        RgbFilter::Green => RgbFilter::green(pixel),
        RgbFilter::Blue => RgbFilter::blue(pixel),
        RgbFilter::Magenta => RgbFilter::magenta(pixel),
        RgbFilter::Cyan => RgbFilter::cyan(pixel),
        RgbFilter::Yellow => RgbFilter::yellow(pixel),
        RgbFilter::SorteColors => RgbFilter::sorted_colors(pixel),
        RgbFilter::SortedColorsRev => RgbFilter::sorted_colors_rev(pixel),
        RgbFilter::SwapRgbColorsI => RgbFilter::swap_rgb_colors_i(pixel),
        RgbFilter::SwapRgbColorsII => RgbFilter::swap_rgb_colors_ii(pixel),
        RgbFilter::SwapRgbColorsIII => RgbFilter::swap_rgb_colors_iii(pixel),
        RgbFilter::RgbNot => RgbFilter::rgb_not(pixel),
        RgbFilter::RgbShlOnce => RgbFilter::rgb_shl_once(pixel),
        RgbFilter::RgbShrOnce => RgbFilter::rgb_shr_once(pixel),
        RgbFilter::RgbShlNth(times) => RgbFilter::rgb_shl_nth(pixel, times),
        RgbFilter::RgbShrNth(times) => RgbFilter::rgb_shr_nth(pixel, times),
        RgbFilter::RgbAndMask(mask) => RgbFilter::rgb_and_mask(pixel, mask),
        RgbFilter::RgbOrMask(mask) => RgbFilter::rgb_or_mask(pixel, mask),
        RgbFilter::RgbXorMask(mask) => RgbFilter::rgb_xor_mask(pixel, mask),
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

    pub fn swap_rgb_colors_i(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[2], rgb[0], rgb[1]])
    }

    pub fn swap_rgb_colors_ii(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[1], rgb[0], rgb[2]])
    }

    pub fn swap_rgb_colors_iii(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], rgb[2], rgb[1]])
    }

    pub fn rgb_and_mask(rgb: &mut Rgb<u8>, mask: &Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] & mask.0[0], rgb[1] & mask.0[1], rgb[2] & mask.0[2]])
    }

    pub fn rgb_or_mask(rgb: &mut Rgb<u8>, mask: &Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] | mask.0[0], rgb[1] | mask.0[1], rgb[2] | mask.0[2]])
    }

    pub fn rgb_xor_mask(rgb: &mut Rgb<u8>, mask: &Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] ^ mask.0[0], rgb[1] ^ mask.0[1], rgb[2] ^ mask.0[2]])
    }

    pub fn rgb_not(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([!rgb[0], !rgb[1], !rgb[2]])
    }

    pub fn rgb_shl_once(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] << 1, rgb[1] << 1, rgb[2] << 1])
    }

    pub fn rgb_shr_once(rgb: &mut Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] >> 1, rgb[1] >> 1, rgb[2] >> 1])
    }

    pub fn rgb_shl_nth(rgb: &mut Rgb<u8>, times: &u8) -> Rgb<u8> {
        Rgb([rgb[0] << times, rgb[1] << times, rgb[2] << times])
    }

    pub fn rgb_shr_nth(rgb: &mut Rgb<u8>, times: &u8) -> Rgb<u8> {
        Rgb([rgb[0] >> times, rgb[1] >> times, rgb[2] >> times])
    }
}
