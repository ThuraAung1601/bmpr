use image::{Pixel, imageops, DynamicImage, GenericImageView, ImageBuffer};

#[allow(dead_code)]
pub struct BmpHeader {
    width: u32,
    height: u32,
    color_depth: u16,
}

#[allow(irrefutable_let_patterns)]
pub fn print_pixel_values(image: &DynamicImage) {
    // Get the image dimensions (width and height)
    let (width, height) = image.dimensions();

    println!("Image Size: {}x{}", width, height);
    // Extract pixel values
    for y in 0..height {
        for x in 0..width {
            // Get the pixel value at (x, y)
            let pixel = image.get_pixel(x, y);
            // Extract RGB or grayscale values (depending on the image type)
            if let rgb = pixel.to_rgb() {
                // RGB format
                let (r, g, b) = (rgb[0], rgb[1], rgb[2]);
                println!("Pixel at ({}, {}): R={}, G={}, B={}", x, y, r, g, b);
            } else if let gray = pixel.to_luma() {
                // Grayscale format
                let intensity = gray[0];
                println!("Pixel at ({}, {}): Gray Intensity={}", x, y, intensity);
            }
        }
    }
}

pub fn display_dimensions(image: &DynamicImage) {
    // Get the image dimensions (width and height)
    let (width, height) = image.dimensions();
    println!("Image Size: {}x{}", width, height);
}

pub fn apply_grayscale(image: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(imageops::colorops::grayscale(image))
}

pub fn scale_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, imageops::FilterType::Lanczos3)
}

pub fn rotate_image(image: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageRgba8(imageops::rotate90(image))
}

pub fn rotate_image_degree(image: &DynamicImage, degrees: f32) -> DynamicImage {
    // Calculate the rotation angle in radians
    let radians = degrees.to_radians();

    // Get the dimensions of the original image
    let (width, height) = image.dimensions();

    // Calculate the center of rotation
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    // Create a new ImageBuffer for the rotated image
    let mut rotated_buffer = ImageBuffer::new(width, height);

    // Iterate through the pixels of the rotated image
    for y in 0..height {
        for x in 0..width {
            // Calculate the corresponding pixel position in the original image after rotation
            let original_x = (x as f32 - center_x) * radians.cos() - (y as f32 - center_y) * radians.sin() + center_x;
            let original_y = (x as f32 - center_x) * radians.sin() + (y as f32 - center_y) * radians.cos() + center_y;

            // Check if the calculated position is within the bounds of the original image
            if original_x >= 0.0 && original_x < width as f32 && original_y >= 0.0 && original_y < height as f32 {
                // Sample the pixel from the original image and copy it to the rotated image
                let pixel = image.get_pixel(original_x as u32, original_y as u32);
                rotated_buffer.put_pixel(x, y, pixel.clone());
            }
        }
    }

    // Convert the ImageBuffer back to a DynamicImage
    DynamicImage::ImageRgba8(rotated_buffer)
}

pub fn threshold(image: &DynamicImage, threshold_value: u8) -> DynamicImage {
    let grayscale_image = image.to_luma8();
    let mut binary_image = grayscale_image.clone();

    for (_, _, pixel) in binary_image.enumerate_pixels_mut() {
        if pixel[0] > threshold_value {
            pixel[0] = 255; // White
        } else {
            pixel[0] = 0; // Black
        }
    }

    DynamicImage::ImageLuma8(binary_image)
}

pub fn apply_blur(image: &DynamicImage) -> DynamicImage {
    image.blur(3.0) // Adjust blur radius as needed
}

pub fn apply_sharpen(image: &DynamicImage) -> DynamicImage {
    // sigma: f32, threshold: i32
    image.unsharpen(3.0, 1) // Adjust sharpening parameters as needed
}

pub fn apply_noise_reduction(image: DynamicImage) -> DynamicImage {
    let image_buffer = image.to_rgba8();
    image::DynamicImage::ImageRgba8(imageproc::filter::gaussian_blur_f32(&image_buffer, 2.0))
}