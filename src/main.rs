extern crate image;
use image::{Pixel, Rgb, RgbImage};
use std::time::Instant;

pub struct MyRgbImage {
    img: RgbImage,
}

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

    pub fn blend_segment(
        &self,
        mut segment: Vec<Rgb<u8>>,
        blender: fn(&mut Rgb<u8>) -> Rgb<u8>,
    ) -> Vec<Rgb<u8>> {
        segment.iter_mut().map(|i| blender(i)).collect::<Vec<_>>()
    }

    pub fn blend_line(&mut self, line: u32, pixel_line: Vec<Rgb<u8>>) {
        (0..self.img.height())
            .for_each(|i | self.img[(line, i)] = pixel_line[i as usize]);
    }

    pub fn blend_colum(&mut self, column: u32, pixel_column: Vec<Rgb<u8>>) {
        (0..self.img.width())
            .for_each(|i| self.img[(i, column)] = pixel_column[i as usize]);
    }

    
    pub fn swap_lines(&mut self, line1: u32, line2: u32) {
        let line_1 = self.get_line(line2);
        let line_2 = self.get_line(line1);
        self.blend_line(line1, line_2);
        self.blend_line(line2, self.blend_segment(line_1,magenta));
    }
    
    pub fn swap_columns(&mut self, column1: u32, column2: u32) {
        let column_1 = self.get_column(column2);
        let column_2 = self.get_column(column1);
        self.blend_colum(column1, column_2);
        self.blend_colum(column2, self.blend_segment(column_1, sorted_colors));
    }
    
    pub fn mess_everything(&mut self) {
        (0..self.img.width() / 2)
        .zip(self.img.width()/2..self.img.width())
        .for_each(|i| self.swap_lines(i.0, i.1));
        (0..self.img.height() / 2)
        .zip((self.img.height()/2)..self.img.height())
        .for_each(|i| self.swap_columns(i.0, i.1));
    }

    pub fn save_image(&self, path: &str) {
        self.img.save(path).expect("algo n deu certo.");
    }
}

pub fn blue(rgb: &mut Rgb<u8>) {
    rgb.blend(&Rgb([rgb[0], rgb[1], 255]))
}

pub fn red(rgb: &mut Rgb<u8>) {
    rgb.blend(&Rgb([255, rgb[1], rgb[2]]))
}

pub fn green(rgb: &mut Rgb<u8>) {
    rgb.blend(&Rgb([rgb[0], 255, rgb[2]]))
}

pub fn magenta(rgb: &mut Rgb<u8>) -> Rgb<u8> {
    Rgb([255, rgb[1], 255])
}

pub fn cyan(rgb: &Rgb<u8>) -> Rgb<u8> {
    Rgb([rgb[0], 255, 255])
}

pub fn yellow(rgb: &mut Rgb<u8>) {
    rgb.blend(&Rgb([255, 255, rgb[2]]))
}

pub fn sorted_colors(rgb: &mut Rgb<u8>) -> Rgb<u8> {
    rgb.0.sort_unstable();
    Rgb([rgb[0], rgb[1], rgb[2]])
}

pub fn swap_rgb_colors(rgb: &mut Rgb<u8>) -> Rgb<u8> {
    Rgb([rgb[2], rgb[0], rgb[1]])
}

pub fn main() {
    let neko_image = image::open("./neko.jpg")
        .expect("Erro ao abrir imagem")
        .to_rgb8();
    let tempo = Instant::now();
    let mut my_image = MyRgbImage::new(neko_image);
    my_image.mess_everything();
    my_image.save_image("./new_neko.jpg");
    println!("saca só essa execução: {:?}", tempo.elapsed());
}
