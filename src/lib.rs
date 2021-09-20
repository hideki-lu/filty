mod error;
mod my_image;

#[cfg(test)]
mod test {
    use crate::error::Error;
    use crate::my_image::MyRgbImage;
    use image::{ImageBuffer, Rgb};

    fn create_sample(width: u32, height: u32) -> MyRgbImage {
        let mut sample = ImageBuffer::from_pixel(width, height, Rgb([0u8, 0u8, 0u8]));
        let columns = sample.width();
        for (x, y, pixel) in sample.enumerate_pixels_mut() {
            *pixel = Rgb([
                pixel.0[0] + (x + y * columns) as u8,
                pixel.0[1] + (x + y * columns) as u8,
                pixel.0[2] + (x + y * columns) as u8,
            ]);
        }
        MyRgbImage::new(sample)
    }

    #[test]
    fn test_error_out_of_bounds_line() {
        assert!(matches!(
            create_sample(1, 5).get_line(1),
            Err(Error::IndexOutOfBounds)
        ));
    }

    #[test]
    fn test_error_out_of_bounds_column() {
        assert!(matches!(
            create_sample(5, 1).get_column(1),
            Err(Error::IndexOutOfBounds)
        ));
    }

    #[test]
    fn test_getting_single_line_image() {
        debug_assert_eq!(
            create_sample(5, 1).get_line(0).unwrap().as_slice(),
            &[
                Rgb([0u8, 0u8, 0u8]),
                Rgb([1u8, 1u8, 1u8]),
                Rgb([2u8, 2u8, 2u8]),
                Rgb([3u8, 3u8, 3u8]),
                Rgb([4u8, 4u8, 4u8])
            ]
        );
    }

    #[test]
    fn test_getting_single_column_image() {
        debug_assert_eq!(
            create_sample(1, 5).get_column(0).unwrap().as_slice(),
            &[
                Rgb([0u8, 0u8, 0u8]),
                Rgb([1u8, 1u8, 1u8]),
                Rgb([2u8, 2u8, 2u8]),
                Rgb([3u8, 3u8, 3u8]),
                Rgb([4u8, 4u8, 4u8])
            ]
        );
    }

    #[test]
    fn test_getting_line() {
        assert_eq!(
            create_sample(3, 3).get_line(1).unwrap().as_slice(),
            &[
                Rgb([3u8, 3u8, 3u8]),
                Rgb([4u8, 4u8, 4u8]),
                Rgb([5u8, 5u8, 5u8]),
            ]
        );
    }

    #[test]
    fn test_getting_column() {
        assert_eq!(
            create_sample(3, 3).get_column(1).unwrap().as_slice(),
            &[
                Rgb([1u8, 1u8, 1u8]),
                Rgb([4u8, 4u8, 4u8]),
                Rgb([7u8, 7u8, 7u8]),
            ]
        );
    }

    #[test]
    fn test_getting_lines_interval() {
        assert_eq!(
            create_sample(3, 3).get_lines_interval(0, 1),
            &[
                Rgb([0u8, 0u8, 0u8]),
                Rgb([1u8, 1u8, 1u8]),
                Rgb([2u8, 2u8, 2u8]),
                Rgb([3u8, 3u8, 3u8]),
                Rgb([4u8, 4u8, 4u8]),
                Rgb([5u8, 5u8, 5u8]),
            ]
        )
    }
    #[test]
    fn test_getting_columns_interval() {
        assert_eq!(
            create_sample(3, 3).get_columns_interval(0, 1),
            &[
                Rgb([0u8, 0u8, 0u8]),
                Rgb([3u8, 3u8, 3u8]),
                Rgb([6u8, 6u8, 6u8]),
                Rgb([1u8, 1u8, 1u8]),
                Rgb([4u8, 4u8, 4u8]),
                Rgb([7u8, 7u8, 7u8]),
            ]
        )
    }

        #[test]
    fn test_getting_lines_full_interval() {
        debug_assert_eq!(create_sample(5, 4).get_lines_interval(0,3), 
        &[
            Rgb([00u8,00u8,00u8]), Rgb([01u8,01u8,01u8]), Rgb([02u8,02u8,02u8]), Rgb([03u8,03u8,03u8]), Rgb([04u8,04u8,04u8]),
            Rgb([05u8,05u8,05u8]), Rgb([06u8,06u8,06u8]), Rgb([07u8,07u8,07u8]), Rgb([08u8,08u8,08u8]), Rgb([09u8,09u8,09u8]),
            Rgb([10u8,10u8,10u8]), Rgb([11u8,11u8,11u8]), Rgb([12u8,12u8,12u8]), Rgb([13u8,13u8,13u8]), Rgb([14u8,14u8,14u8]),
            Rgb([15u8,15u8,15u8]), Rgb([16u8,16u8,16u8]), Rgb([17u8,17u8,17u8]), Rgb([18u8,18u8,18u8]), Rgb([19u8,19u8,19u8]),
         ]
    )
    }
}
