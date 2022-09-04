use image::Rgb;
use filty::my_image::MyRgbImage;
use filty::my_image::RgbFilter;

fn main() -> filty::error::Result<()> {
    let neko_image = image::open("./neko.jpg")
        .expect("Erro ao abrir imagem")
        .to_rgb8();

    MyRgbImage::new(neko_image)
        .blend(RgbFilter::RgbXorMask(Rgb([0, 125, 125])))
        .save_image("./new_neko.jpg")?;

    Ok(())
}
