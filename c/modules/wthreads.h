#ifndef WTHREADS_H_   /* guard */
#define WTHREADS_H_

double time_threads(int width, int height, double complex upper_left, double complex lower_right, int number_of_threads, int draw);
int measure_workload_threads(int width, int height, double complex upper_left, double complex lower_right);

#endif
