#ifndef WTHREADS_H_
#define WTHREADS_H_

// Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
// just using threads from the standard library
// Return the ms on succes or -1 on failure
// Arguments:
// width, height of the image in pixels
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
// number_of_threads specify the number of bands.
// draw decides whether to write the computed mandelbrot set to png or not.
double time_threads(int width, int height, double complex upper_left, double complex lower_right, int number_of_threads, int draw);

// Measures how long it takes for every number of threads in a
// range from 4 to 80 by repeating the measurement 20 times each. The results are written to a file.
// Return 0 on success and -1 on failure.

// Arguments
// width, height of the image in pixels
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
int measure_workload_threads(int width, int height, double complex upper_left, double complex lower_right);

#endif
