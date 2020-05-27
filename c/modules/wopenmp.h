#ifndef WOPENMP_H_
#define WOPENMP_H_

// Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
// using the openmp library.
// Return the ms on succes or -1 on failure
// Arguments:
// width, height of the image in pixels
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
// rows_per_band divided by the height gives the number of bands
// number_of_threads the number of threads to run with
// draw decides whether to write the computed mandelbrot set to png or not.
double time_openmp(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band, int number_of_threads, int draw);

// Measures how long it takes for every number of rows_per_band in a
// range from 1 to 80 by repeating the measurement 20 times each. The results are written to a file.
// Return 0 on success and -1 on failure.

// Arguments
// width, height of the image in pixels
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
// rows_per_band divided by the height gives the number of bands. For best results the best value is probably between 1 and 5.
int measure_workload_openmp(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band);

#endif
