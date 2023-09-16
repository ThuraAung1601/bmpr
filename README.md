# bmpr
CLI-based BMP Image Manipulator

### Usage
```{r, engine='bash', count_lines}
BMP Image Manipulator 1.0
Thura Aung
For BMP image manipulation

USAGE:
    bmpr [FLAGS] [OPTIONS] --input <INPUT_FILE> --output <OUTPUT_FILE>

FLAGS:
    -b, --blur                  Apply blur to the image
    -d, --display-dimensions    Display image dimensions
    -g, --grayscale             Apply grayscale
    -h, --help                  Prints help information
        --noise-reduction       Apply noise reduction to the image
    -p, --print-pixels          Print pixel values
    -x, --sharpen               Apply sharpening to the image
    -V, --version               Prints version information

OPTIONS:
    -i, --input <INPUT_FILE>             Input BMP file
    -o, --output <OUTPUT_FILE>           Output BMP file [default: output.bmp]
    -r, --rotate <DEGREE>                Rotate the image to the specified degree
    -s, --scale <WIDTH,HEIGHT>           Scale the image to the specified width and height
    -t, --threshold <THRESHOLD_VALUE>    Threshold value for thresholding
```
