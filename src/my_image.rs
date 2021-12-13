use crate::error::Result;
use image::{Rgb, RgbImage};
use std::{ops::Range, path::Path};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PairSelection {
    Lines,
    Columns,
}
enum PixelSelection {
    All,
    Column(u32),
    Line(u32),
    LinesRange(Range<u32>),
    ColumnsRange(Range<u32>),
    Function(fn(u32, u32) -> (u32, u32)),
    Pair(u32, u32, PairSelection),
}
pub struct MyRgbImage {
    selection: PixelSelection,
    img: RgbImage,
}

fn apply_all(image: &mut RgbImage, rgb_filter: RgbFilter) {
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = apply_filter(rgb_filter, *pixel));
}

fn apply_line(image: &mut RgbImage, line: u32, rgb_filter: RgbFilter) {
    (0..image.width()).for_each(|i| image[(i, line)] = apply_filter(rgb_filter, image[(i, line)]))
}

fn apply_column(image: &mut RgbImage, column: u32, rgb_filter: RgbFilter) {
    (0..image.height())
        .for_each(|i| image[(column, i)] = apply_filter(rgb_filter, image[(column, i)]))
}

fn apply_columns(image: &mut RgbImage, columns: Range<u32>, rgb_filter: RgbFilter) {
    columns.for_each(|column| {
        (0..image.height())
            .for_each(|i| image[(column, i)] = apply_filter(rgb_filter, image[(column, i)]))
    })
}

fn apply_lines(image: &mut RgbImage, lines: Range<u32>, rgb_filter: RgbFilter) {
    lines.for_each(|line| {
        (0..image.width())
            .for_each(|i| image[(i, line)] = apply_filter(rgb_filter, image[(i, line)]))
    })
}

fn apply_function(
    image: &mut RgbImage,
    function: fn(u32, u32) -> (u32, u32),
    rgb_filter: RgbFilter,
) {
    let (width, height) = (image.width(), image.height());
    image
        .enumerate_pixels_mut()
        .filter(|pixel| {
            let (new_x, new_y) = function(pixel.0, pixel.1);
            new_x < width && new_y < height
        })
        .for_each(|i| *i.2 = apply_filter(rgb_filter, *i.2));
}

fn apply_pair(
    image: &mut RgbImage,
    one_segment: u32,
    other_segment: u32,
    pair_selection: PairSelection,
    rgb_filter: RgbFilter,
) {
    match pair_selection {
        PairSelection::Lines => (0..image.width()).for_each(|i| {
            image[(i, one_segment)] = apply_filter(rgb_filter, image[(i, one_segment)]);
            image[(i, other_segment)] = apply_filter(rgb_filter, image[(i, other_segment)]);
        }),
        PairSelection::Columns => (0..image.height()).for_each(|i| {
            image[(one_segment, i)] = apply_filter(rgb_filter, image[(one_segment, i)]);
            image[(other_segment, i)] = apply_filter(rgb_filter, image[(other_segment, i)]);
        }),
    }
}

#[allow(dead_code)]
impl MyRgbImage {
    pub fn new(a_image: RgbImage) -> Self {
        MyRgbImage {
            selection: PixelSelection::All,
            img: a_image,
        }
    }

    pub fn for_line(mut self, line: u32) -> Self {
        self.selection = PixelSelection::Line(line);
        self
    }

    pub fn for_column(mut self, column: u32) -> Self {
        self.selection = PixelSelection::Column(column);
        self
    }

    pub fn for_lines(mut self, lines: Range<u32>) -> Self {
        self.selection = PixelSelection::LinesRange(lines);
        self
    }

    pub fn for_columns(mut self, columns: Range<u32>) -> Self {
        self.selection = PixelSelection::ColumnsRange(columns);
        self
    }

    pub fn for_function(mut self, function: fn(u32, u32) -> (u32, u32)) -> Self {
        self.selection = PixelSelection::Function(function);
        self
    }

    pub fn for_pair(
        mut self,
        one_segment: u32,
        other_segment: u32,
        pair_selection: PairSelection,
    ) -> Self {
        self.selection = PixelSelection::Pair(one_segment, other_segment, pair_selection);
        self
    }

    pub fn swap_pair(self) -> Self {
        match self.selection {
            PixelSelection::Pair(one_segment, other_segment, pair_selection) => {
                match pair_selection {
                    PairSelection::Lines => self.swap_lines(one_segment, other_segment),
                    PairSelection::Columns => self.swap_columns(one_segment, other_segment),
                }
            }
            _ => self,
        }
    }

    fn swap_lines(mut self, one_line: u32, other_line: u32) -> Self {
        (0..self.img.width()).for_each(|i| {
            let pixel = self.img[(i, one_line)];
            self.img[(i, one_line)] = self.img[(i, other_line)];
            self.img[(i, other_line)] = pixel;
        });
        self
    }

    fn swap_columns(mut self, one_column: u32, other_column: u32) -> Self {
        (0..self.img.height()).for_each(|i| {
            let pixel = self.img[(one_column, i)];
            self.img[(one_column, i)] = self.img[(other_column, i)];
            self.img[(other_column, i)] = pixel;
        });
        self
    }

    pub fn blend(mut self, rgb_filter: RgbFilter) -> Self {
        match &self.selection {
            PixelSelection::All => apply_all(&mut self.img, rgb_filter),
            PixelSelection::Line(line) => apply_line(&mut self.img, *line, rgb_filter),
            PixelSelection::Column(column) => apply_column(&mut self.img, *column, rgb_filter),
            PixelSelection::LinesRange(lines) => {
                apply_lines(&mut self.img, lines.to_owned(), rgb_filter)
            }
            PixelSelection::ColumnsRange(columns) => {
                apply_columns(&mut self.img, columns.to_owned(), rgb_filter)
            }
            PixelSelection::Function(function) => {
                apply_function(&mut self.img, *function, rgb_filter)
            }
            PixelSelection::Pair(one_segment, other_segment, pair_selection) => apply_pair(
                &mut self.img,
                *one_segment,
                *other_segment,
                *pair_selection,
                rgb_filter,
            ),
        }
        self
    }

    pub fn mess_everything(mut self) -> MyRgbImage {
        let (height, width) = (self.img.height(), self.img.width());
        self = self
            .for_lines(0..height / 4)
            .blend(RgbFilter::Magenta)
            .blend(RgbFilter::RgbXorMask(Rgb([0, 20, 50])))
            .for_columns(0..width / 4)
            .blend(RgbFilter::Cyan)
            .for_columns(0..width / 2)
            .blend(RgbFilter::RgbNot)
            .blend(RgbFilter::RgbShlOnce);
        let line_pairs = (0..height)
            .filter(|i| i % 4 == 0)
            .zip((0..height).filter(|i| i % 4 == 3));
        for (i, j) in line_pairs {
            self = self
                .for_pair(i, j, PairSelection::Lines)
                .blend(RgbFilter::SorteColors)
                .swap_lines(i, j)
        }
        self
    }

    pub fn save_image<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Ok(self.img.save(path)?)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
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
pub fn apply_filter(filter: RgbFilter, pixel: Rgb<u8>) -> Rgb<u8> {
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

impl RgbFilter {
    fn blue(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], rgb[1], 255])
    }

    fn red(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([255, rgb[1], rgb[2]])
    }

    fn green(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], 255, rgb[2]])
    }

    fn magenta(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([255, rgb[1], 255])
    }

    fn cyan(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], 255, 255])
    }

    fn yellow(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([255, 255, rgb[2]])
    }

    fn sorted_colors(mut rgb: Rgb<u8>) -> Rgb<u8> {
        rgb.0.sort_unstable();
        Rgb([rgb[0], rgb[1], rgb[2]])
    }

    fn sorted_colors_rev(mut rgb: Rgb<u8>) -> Rgb<u8> {
        rgb.0.sort_unstable();
        rgb.0.reverse();
        Rgb([rgb[0], rgb[1], rgb[2]])
    }

    fn swap_rgb_colors_i(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[2], rgb[0], rgb[1]])
    }

    fn swap_rgb_colors_ii(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[1], rgb[0], rgb[2]])
    }

    fn swap_rgb_colors_iii(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0], rgb[2], rgb[1]])
    }

    fn rgb_and_mask(rgb: Rgb<u8>, mask: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] & mask.0[0], rgb[1] & mask.0[1], rgb[2] & mask.0[2]])
    }

    fn rgb_or_mask(rgb: Rgb<u8>, mask: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] | mask.0[0], rgb[1] | mask.0[1], rgb[2] | mask.0[2]])
    }

    fn rgb_xor_mask(rgb: Rgb<u8>, mask: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] ^ mask.0[0], rgb[1] ^ mask.0[1], rgb[2] ^ mask.0[2]])
    }

    fn rgb_not(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([!rgb[0], !rgb[1], !rgb[2]])
    }

    fn rgb_shl_once(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] << 1, rgb[1] << 1, rgb[2] << 1])
    }

    fn rgb_shr_once(rgb: Rgb<u8>) -> Rgb<u8> {
        Rgb([rgb[0] >> 1, rgb[1] >> 1, rgb[2] >> 1])
    }

    fn rgb_shl_nth(rgb: Rgb<u8>, times: u8) -> Rgb<u8> {
        Rgb([rgb[0] << times, rgb[1] << times, rgb[2] << times])
    }

    fn rgb_shr_nth(rgb: Rgb<u8>, times: u8) -> Rgb<u8> {
        Rgb([rgb[0] >> times, rgb[1] >> times, rgb[2] >> times])
    }
}
