mod error;
mod my_image;

#[cfg(test)]
mod test {
    use crate::my_image::MyRgbImage;
    use image::{ImageBuffer, Rgb};

    pub fn create_sample(width: u32, height: u32) -> MyRgbImage {
        let mut sample = ImageBuffer::from_pixel(width, height, Rgb([0u8, 0u8, 0u8]));
        let height = sample.height();
        for (x, y, pixel) in sample.enumerate_pixels_mut() {
            *pixel = Rgb([
                pixel.0[0] + (x + y * height) as u8 * 5,
                pixel.0[1] + (x + y * height) as u8 * 5,
                pixel.0[2] + (x + y * height) as u8 * 5,
            ]);
        }
        MyRgbImage::new(sample)
    }

    #[test]
    fn test_getting_line() {
        let line = create_sample(3, 3);
        assert_eq!(
            line.get_line(0).unwrap().as_slice(),
            &[
                Rgb([0u8, 0u8, 0u8]),
                Rgb([5u8, 5u8, 5u8]),
                Rgb([10u8, 10u8, 10u8]),
            ]
        );
    }

    #[test]
    fn test_getting_column() {
        let column = create_sample(3, 3);
        assert_eq!(
            column.get_column(0).unwrap().as_slice(),
            &[
                Rgb([0u8, 0u8, 0u8]),
                Rgb([15u8, 15u8, 15u8]),
                Rgb([30u8, 30u8, 30u8]),
            ]
        );
    }

    #[test]
    fn test_getting_lines_interval() {
        let lines = create_sample(3, 3);
        assert_eq!(lines.get_lines_interval(0, 1),
        &[
            Rgb([0u8, 0u8, 0u8]),
            Rgb([5u8, 5u8, 5u8]),
            Rgb([10u8, 10u8, 10u8]),
            Rgb([15u8, 15u8, 15u8]),
            Rgb([20u8, 20u8, 20u8]),
            Rgb([25u8, 25u8, 25u8]),

        ])
    }
}
