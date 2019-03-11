# edgy
A simple program to apply the Sobel edge detection operator to any given image or directory of images.

# Installation 
The process to install is pretty standard:
```bash
$ cargo install edgy
```

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
# Examples

![Pillow Man](https://raw.githubusercontent.com/dangreco/edgy/master/examples/PillowManEx.jpg)

![Donald Glover](https://raw.githubusercontent.com/dangreco/edgy/master/examples/DonaldGloverEx.jpg)

![Detour Sign](https://raw.githubusercontent.com/dangreco/edgy/master/examples/DetourEx.jpg)

# Performance
Running this implementation on the same image but scaled down to various sizes, I charted the time it took to produce the final image (in milliseconds).

![Performance](https://raw.githubusercontent.com/dangreco/edgy/master/examples/Performance.jpg)


# Contributions
If you want to improve anything, go ahead and open up a pull request.
