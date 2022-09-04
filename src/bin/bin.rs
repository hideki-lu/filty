extern crate filty;

use filty::my_image::{MyRgbImage, Point, RgbFilter};
use image::Rgb;
use std::time::Instant;

pub fn main() -> filty::error::Result<()> {
    let neko_image = image::open("./neko.jpg")
        .expect("Erro ao abrir imagem")
        .to_rgb8();
    let tempo = Instant::now();
    MyRgbImage::new(neko_image)
        .for_all()
        .blend(RgbFilter::Solid(Rgb([0, 0, 0])))
        .draw_triangule(
            Point::new(101, 192),
            Point::new(154, 183),
            Point::new(221 , 253),
            RgbFilter::RgbNot
        )
        .save_image("./new_neko.jpg")?;
    println!("saca só essa execução: {:?}", tempo.elapsed());

    Ok(())
}
