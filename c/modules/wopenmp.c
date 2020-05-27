#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include "mandel.h"
#include <omp.h>

double time_openmp(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band, int number_of_threads, int draw) {
        char *pixels;
        int i, band_len, arr_len, num_of_jobs;
        struct timespec start, end;

        arr_len = width * height;
        //if rows_per_band doesn't fit perfectly in height without rest, it must be round upward to make sure that the bands cover the entire image.
        num_of_jobs = height % rows_per_band == 0 ? height / rows_per_band : height / rows_per_band + 1;
        band_len = rows_per_band * width;

        pixels = (char*)malloc(arr_len * sizeof(char));
        if(!pixels) {
                perror("malloc failed");
                return -1.0;
        }

        if(clock_gettime(CLOCK_MONOTONIC_RAW, &start) == -1) {
                perror("clock gettime failed");
                free(pixels);
                return -1.0;
        }

        //A much more simple solution would be:
        //#pragma omp parallel for num_threads(number_of_threads)
        //The implemented version is a good practice version. The programmer
        //has to think about which variables will be shared between the threads.
        #pragma omp parallel for default(none) num_threads(number_of_threads) shared(pixels, num_of_jobs, band_len, arr_len, rows_per_band, width, height, upper_left, lower_right)
        for(i = 0; i < num_of_jobs; i++) {
                int offset = band_len * i;
                //in case of last band is smaller than the previous ones.
                int check_band_len = arr_len - offset > band_len ? band_len : arr_len - offset;
                int top = rows_per_band * i;
                int band_height = check_band_len / width;
                double complex band_upper_left = pixel_to_point(width, height, 0, top, upper_left, lower_right);
                double complex band_lower_right = pixel_to_point(width, height, width, top + band_height, upper_left, lower_right);

                render_openmp(pixels + offset, width, band_height, band_upper_left, band_lower_right);
        }

        if(clock_gettime(CLOCK_MONOTONIC_RAW, &end) == -1) {
                perror("clock gettime failed");
                free(pixels);
                return -1.0;
        }

        if(draw) {
                if(write_image("mandel.png", pixels, width, height) == -1) {
                        perror("write image failed");
                        free(pixels);
                        return -1.0;
                }
        }

        return compute_time_milis(start, end);
}

int measure_workload_openmp(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band) {
        FILE *fp;
        fp = fopen("c_openmp_performance.txt", "w");
        if (!fp) {
                perror("Could not open file \"c_openmp_performance.txt\"");
                fclose(fp);
                return -1;
        }

        for(int thread_count = 4; thread_count <= 80; thread_count++) {
                double time = 0;
                for(int i = 0; i < 20; i++) {
                        double res = time_openmp(width, height, upper_left, lower_right, rows_per_band, thread_count, 0);
                        if(res == -1.0) {
                                perror("time with openmp failed");
                                fclose(fp);
                                return -1;
                        }
                        time += res;
                }
                time /= 20;
                int printed = fprintf(fp, "%d,%f\n", thread_count, time);
                if(printed == 0) {
                        perror("Write to file failed");
                        fclose(fp);
                        return -1;
                }
        }
        fclose(fp);

        return 0;
}
