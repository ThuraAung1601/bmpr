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
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    
    // Load the input BMP image
    let mut input_image = image::open(input_file)?;

    if matches.is_present("grayscale") {
        // Apply grayscale
        input_image = bmpr::apply_grayscale(&mut input_image);
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
            input_image = bmpr::scale_image(&input_image, width, height);
        } else {
            eprintln!("Invalid number of arguments for scale. Use -s <width> <height>");
            std::process::exit(1);
        }
    }

    if let Some(degree) = matches.value_of("rotate") {
        let degree = degree.parse::<f32>().expect("Invalid width value");
        // Rotating image with specified degree
        input_image = bmpr::rotate_image_degree(&input_image, degree);
    }

    // Save the manipulated image
    input_image
        .save_with_format(output_file, ImageFormat::Bmp)
        .expect("Failed to save manipulated image");

    Ok(())
}