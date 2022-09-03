extern crate filty;

use filty::my_image::MyRgbImage;
use std::time::Instant;

pub fn main() -> filty::error::Result<()> {
    let neko_image = image::open("./neko.jpg")
        .expect("Erro ao abrir imagem")
        .to_rgb8();
    let tempo = Instant::now();
    MyRgbImage::new(neko_image)
        .mess_everything()
        .save_image("./new_neko.jpg")?;
    println!("saca só essa execução: {:?}", tempo.elapsed());

    Ok(())
}
