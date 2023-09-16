use clap::{Arg, App};
use image::ImageFormat;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("BMP Image Manipulator")
        .version("1.0")
        .author("Thura Aung")
        .about("For BMP image manipulation")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT_FILE")
                .help("Input BMP file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output BMP file")
                .required(true)
                .takes_value(true)
                .default_value("output.bmp"),
        )
        .arg(
            Arg::with_name("print_pixels")
                .short("p")
                .long("print-pixels")
                .help("Print pixel values")
                .conflicts_with("display_dimensions"),
        )
        .arg(
            Arg::with_name("display_dimensions")
                .short("d")
                .long("display-dimensions")
                .help("Display image dimensions")
                .conflicts_with("print_pixels"),
        )
        .arg(
            Arg::with_name("grayscale")
                .short("g")
                .long("grayscale")
                .help("Apply grayscale")
                .required(false),
        )
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .value_name(&"WIDTH,HEIGHT")
                .help("Scale the image to the specified width and height")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rotate")
                .short("r")
                .long("rotate")
                .value_name("DEGREE")
                .help("Rotate the image to the specified degree")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threshold")
                .short("t")
                .long("threshold")
                .value_name("THRESHOLD_VALUE")
                .help("Threshold value for thresholding")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("blur")
                .short("b")
                .long("blur")
                .help("Apply blur to the image"),
        )
        .arg(
            Arg::with_name("sharpen")
                .short("x")
                .long("sharpen")
                .help("Apply sharpening to the image"),
        )
        .arg(
            Arg::with_name("noise_reduction")
                .short("n")
                .long("noise-reduction")
                .help("Apply noise reduction to the image"),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    
    // Load the input BMP image
    let input_image = image::open(input_file)?;

    let mut manipulated_image = input_image.clone();

    if matches.is_present("grayscale") {
        // Apply grayscale
        manipulated_image = bmpr::apply_grayscale(&mut manipulated_image);
    }

    if matches.is_present("print_pixels") {
        // Print pixel values
        bmpr::print_pixel_values(&input_image);
    }
    
    if matches.is_present("display_dimensions") {
        // Display image dimensions
        bmpr::display_dimensions(&input_image);
    }

    if let Some(scale_values) = matches.value_of("scale") {
        let values: Vec<&str> = scale_values.split(',').collect();
    
        if values.len() == 2 {
            let width = values[0].parse::<u32>().expect("Invalid width value");
            let height = values[1].parse::<u32>().expect("Invalid height value");
            // Resizing with specified width and height
            manipulated_image = bmpr::scale_image(&manipulated_image, width, height);
        } else {
            eprintln!("Invalid number of arguments for scale. Use -s <width> <height>");
            std::process::exit(1);
        }
    }

    if let Some(degree) = matches.value_of("rotate") {
        let degree = degree.parse::<f32>().expect("Invalid width value");
        // Rotating image with specified degree
        manipulated_image = bmpr::rotate_image_degree(&manipulated_image, degree);
    }

    if matches.is_present("threshold") {
        let threshold_value = matches.value_of("threshold").unwrap_or("128").parse::<u8>()?;
        manipulated_image = bmpr::threshold(&manipulated_image, threshold_value);
    }

    if matches.is_present("blur") {
        manipulated_image = bmpr::apply_blur(&manipulated_image);
    }

    if matches.is_present("sharpen") {
        manipulated_image = bmpr::apply_sharpen(&manipulated_image);
    }

    if matches.is_present("noise_reduction") {
        manipulated_image = bmpr::apply_noise_reduction(manipulated_image);
    }

    // Save the manipulated image
    manipulated_image
        .save_with_format(output_file, ImageFormat::Bmp)
        .expect("Failed to save manipulated image");

    Ok(())
}
