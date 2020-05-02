#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <pthread.h>
#include "mandel.h"
#include "../C-Thread-Pool/thpool.h"

double time_threadpool(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band, int pool_size, int draw) {
        char *pixels;
        int i, chunk_len, arr_len, num_of_jobs;
        double retval;
        struct timespec start, end;
        threadpool thpool;

        arr_len = width * height;
        //if rows_per_band doesn't fit perfectly in height without rest, it must be round upward to make sure that the bands cover the entire image.
        num_of_jobs = height % rows_per_band == 0 ? height / rows_per_band : height / rows_per_band + 1;
        chunk_len = rows_per_band * width;
        render_args* args[num_of_jobs];

        pixels = (char*)malloc(arr_len * sizeof(char));
        if(!pixels) {
                perror("malloc failed");
                retval = -1;
                goto freepixels;
        }

        for(i = 0; i < num_of_jobs; ++i) {
                args[i] = (render_args*)malloc(sizeof(render_args));
                if(!args[i]) {
                        perror("malloc failed");
                        retval = -1;
                        goto freeall;
                }
        }

        if(clock_gettime(CLOCK_MONOTONIC_RAW, &start) == -1) {
                perror("clock gettime failed");
                retval = -1;
                goto freeall;
        }

        //returns void
        thpool = thpool_init(pool_size);

        for(i = 0; i < num_of_jobs; i++) {
                int offset = chunk_len * i;
                //in case of last chunk is smaller than the previous ones.
                int check_chunk_len = arr_len - offset > chunk_len ? chunk_len : arr_len - offset;
                int top = rows_per_band * i;
                int band_height = check_chunk_len / width;
                double complex band_upper_left = pixel_to_point(width, height, 0, top, upper_left, lower_right);
                double complex band_lower_right = pixel_to_point(width, height, width, top + band_height, upper_left, lower_right);

                args[i]->chunk = &pixels[offset];
                args[i]->width = width;
                args[i]->height = band_height;
                args[i]->upper_left = band_upper_left;
                args[i]->lower_right = band_lower_right;

                if(thpool_add_work(thpool, (void*)render, args[i]) == -1) {
                        perror("submit job to the threadpool failed");
                        retval = -1;
                        goto freeall;
                }
        }

        thpool_wait(thpool);


        if(clock_gettime(CLOCK_MONOTONIC_RAW, &end) == -1) {
                perror("clock gettime failed");
                retval = -1;
                goto freeall;
        }

        if(draw) {
                if(write_image("mandel.png", pixels, width, height) == -1) {
                        perror("write image failed");
                        retval = -1;
                        goto freeall;
                }
        }

        retval = compute_time_milis(start, end);

freeall:
        thpool_destroy(thpool);
        for(i = 0; i < num_of_jobs; i++) {
                if(args[i])
                        free(args[i]);
        }
freepixels:
        free(pixels);
        return retval;
}

int measure_workload_threadpool(int width, int height, double complex upper_left, double complex lower_right, int pool_size) {
        FILE *fp;
        fp = fopen("c_threadpool_performance.txt", "w");
        if (!fp) {
                perror("Could not open file \"c_threadpool_performance.txt\"");
                return -1;
        }

        for(int rows_per_band = 1; rows_per_band <= 80; rows_per_band++) {
                double time = 0;
                for(int i = 0; i < 20; i++) {
                        double res = time_threadpool(width, height, upper_left, lower_right, rows_per_band, pool_size, 0);
                        if(res == -1.0) {
                                perror("time with threadpool failed");
                                return -1;
                        }
                        time += res;
                }
                time /= 20;

                int printed = fprintf(fp, "%d,%f\n", rows_per_band, time);
                if(printed == 0) {
                        perror("Write to file failed");
                        return -1;
                }
        }
        fclose(fp);

        return 0;
}
