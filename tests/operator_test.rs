use image::{Rgb, DynamicImage};
use image::GenericImageView;
use image::ImageBuffer;

// Test the `apply_grayscale` function
#[test]
fn test_apply_grayscale() {
    // Create a sample RGB image
    let image = DynamicImage::ImageRgb8(ImageBuffer::from_fn(3, 2, |x, y| {
        Rgb([x as u8, y as u8, (x + y) as u8])
    }));

    // Apply grayscale
    let grayscale_image = bmpr::apply_grayscale(&image);

    // Assert that the resulting image is grayscale
    assert_eq!(grayscale_image.color(), image::ColorType::L8);
}

// Test the `scale_image` function
#[test]
fn test_scale_image() {
    // Create a sample image
    let image = DynamicImage::ImageRgb8(ImageBuffer::from_fn(4, 4, |x, y| {
        Rgb([x as u8, y as u8, (x + y) as u8])
    }));

    // Scale the image to a smaller size
    let scaled_image = bmpr::scale_image(&image, 2, 2);

    // Assert that the resulting image has the expected dimensions
    assert_eq!(scaled_image.width(), 2);
    assert_eq!(scaled_image.height(), 2);
}

// Test the `rotate_image` function
#[test]
fn test_rotate_image() {
    // Create a sample image
    let image = DynamicImage::ImageRgb8(ImageBuffer::from_fn(3, 3, |x, y| {
        Rgb([x as u8, y as u8, (x + y) as u8])
    }));

    // Rotate the image
    let rotated_image = bmpr::rotate_image(&image);

    // Assert that the resulting image has the expected dimensions
    assert_eq!(rotated_image.width(), 3);
    assert_eq!(rotated_image.height(), 3);
}
