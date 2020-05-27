#ifndef MANDEL_H_
#define MANDEL_H_

// Given the row and column of a pixel in the output image, return the
// corresponding point on the complex plane.

// Arguments:
// width, height of the image in pixels
// p_colum, p_row indicating a particular pixel in that image
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
double complex pixel_to_point(int width, int height,
                              int pixel_x, int pixel_y,
                              double complex upper_left,
                              double complex lower_right);

// Render a rectangle of the Mandelbrot set into a buffer of pixels.
// This function takes a pointer to a struct render_args
// It is designed to run in parallel
void *render(void *args);

// Render a rectangle of the Mandelbrot set into a buffer of pixels.
// A modification of the render function which is designed to run with openmp but could also be used in a single thread
// Arguments:
// band is a buffer which holds one grayscale pixel per byte
// width and height specify the width and height of the band
// upper_left and lower_right specify the points on the complex plane designating the corresponding
// upper left and lower right corners of the band.
void render_openmp(char *band, int width, int height, double complex upper_left, double complex lower_right);

// Write an image to a png file.
// Return 0 on success and -1 on failure.
// Arguments:
// filneme specify the name of the file
// A buffer holding one pixel per byte in grayscale
// width and height giving the dimensions of the image in pixels
int write_image(char *filename, char *pixels, int width, int height);

// Computes the passed time between two timestamps in ms
double compute_time_milis(struct timespec start, struct timespec end);

// band is a buffer which holds one grayscale pixel per byte
// width and height specify the width and height of the band
// upper_left and lower_right specify the points on the complex plane designating the corresponding
// upper left and lower right corners of the band.
typedef struct {
        char *band;
        int width;
        int height;
        double complex upper_left;
        double complex lower_right;
} render_args;

#endif // MANDEL_H_
