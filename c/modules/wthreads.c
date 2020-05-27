#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <pthread.h>
#include "mandel.h"

double time_threads(int width, int height, double complex upper_left, double complex lower_right, int number_of_threads, int draw) {
        char *pixels;
        int i, rows_per_band, band_len, arr_len;
        struct timespec start, end;
        pthread_t thread_id[number_of_threads];
        render_args args[number_of_threads];

        arr_len = width * height;
        //if number_of_threads doesn't fit perfectly in height without rest, it must be round upward to make sure that the bands cover the entire image.
        rows_per_band = height % number_of_threads == 0 ? height / number_of_threads : height / number_of_threads + 1;
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

        for(i = 0; i < number_of_threads; i++) {
                int offset = band_len * i;
                //in case of last band is smaller than the previous ones.
                int check_band_len = arr_len - offset > band_len ? band_len : arr_len - offset;
                int top = rows_per_band * i;
                int band_height = check_band_len / width;
                double complex band_upper_left = pixel_to_point(width, height, 0, top, upper_left, lower_right);
                double complex band_lower_right = pixel_to_point(width, height, width, top + band_height, upper_left, lower_right);

                args[i].band = pixels + offset;
                args[i].width = width;
                args[i].height = band_height;
                args[i].upper_left = band_upper_left;
                args[i].lower_right = band_lower_right;

                if(pthread_create(&thread_id[i], NULL, render, &args[i]) != 0) {
                        perror("create thread failed");
                        free(pixels);
                        return -1.0;
                }
        }

        for(i = 0; i < number_of_threads; i++) {
                if(pthread_join(thread_id[i], NULL) != 0) {
                        perror("join thread failed");
                        free(pixels);
                        return -1.0;
                };
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

        free(pixels);
        return compute_time_milis(start, end);
}

int measure_workload_threads(int width, int height, double complex upper_left, double complex lower_right) {
        FILE *fp;
        fp = fopen("c_threads_performance.txt", "w");
        if (!fp) {
                perror("Could not open file for writing \"c_threads_performance.txt\"");
                fclose(fp);
                return -1;
        }

        for(int thread_count = 4; thread_count <= 80; thread_count++) {
                double time = 0;
                for(int i = 0; i < 20; i++) {
                        double res = time_threads(width, height, upper_left, lower_right, thread_count, 0);
                        if(res == -1.0) {
                                perror("time with threads failed");
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
