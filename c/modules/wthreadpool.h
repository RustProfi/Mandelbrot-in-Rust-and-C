#ifndef WTHREADPOOL_H_
#define WTHREADPOOL_H_

// Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
// using an extern threadpool library with free License.
// Return the ms on succes or -1 on failure
// Arguments:
// width, height of the image in pixels
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
// rows_per_band specifies how big a band will be
// pool_size the number of threads the threadpool will be initialized with
// draw decides whether to write the computed mandelbrot set to png or not.
double time_threadpool(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band, int pool_size, int draw);

// Measures how long it takes for every number of rows_per_band in a
// range from 1 to 80 by repeating the measurement 20 times each. The results are written to a file.
// Return 0 on success and -1 on failure.

// Arguments
// width, height of the image in pixels
// upper_left and lower_right specify the points on the complex plane designating the area of the image.
// pool_size the number of threads the threadpool will be initialized with
int measure_workload_threadpool(int width, int height, double complex upper_left, double complex lower_right, int pool_size);

#endif
