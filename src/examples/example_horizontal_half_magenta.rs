use filty::my_image::MyRgbImage;
use filty::my_image::RgbFilter;

pub fn example() -> filty::error::Result<()> {
    let neko_image = image::open("./neko.jpg")
        .expect("Erro ao abrir imagem")
        .to_rgb8();

    MyRgbImage::new(neko_image)
        .for_columns(0..750 / 4)
        .blend(RgbFilter::Magenta)
        .save_image("./new_neko.jpg")?;

    Ok(())
}
