# Edgy
A simple program to apply the Sobel edge detection operator to any given image or directory of images.

# Usage
```
edgy 1.0
Daniel G. <dan.greco@live.com>
Applys Sobel edge detection to given image.
USAGE:
    edgy [FLAGS] [OPTIONS] --input <FILE> --output <FILE>
FLAGS:
    -m, --multiple    Uses input/output as directories; applies sobel operator to every image in that directory.
    -h, --help        Prints help information
    -V, --version     Prints version information
OPTIONS:
    -b, --blur <FACTOR>    Increases blur factor by <FACTOR>
    -i, --input <FILE>     Sets the input image file to use
    -o, --output <FILE>    Sets the output image file to use
```

# Contributions