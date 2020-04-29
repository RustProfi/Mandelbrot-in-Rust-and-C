#ifndef WOPENMP_H_   /* guard */
#define WOPENMP_H_

double time_openmp(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band, int number_of_threads, int draw);
int measure_workload_openmp(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band);

#endif
