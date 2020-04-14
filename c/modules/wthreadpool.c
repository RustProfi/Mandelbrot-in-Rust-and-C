#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <pthread.h>
#include "mandel.h"
#include "../C-Thread-Pool/thpool.h"

//-1.0 in case of error
double time_threadpool(int width, int height, double complex upper_left, double complex lower_right, int rows_per_band, int number_of_threads, int draw) {
        char *pixels;
        int i, offset, chunk_len, arr_len, num_of_jobs;
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
                fprintf(stderr, "malloc failed\n");
                retval = -1;
                goto freepixels;
        }

        for(i = 0; i < num_of_jobs; ++i) {
                args[i] = (render_args*)malloc(sizeof(render_args));
                if(!args[i]) {
                        fprintf(stderr, "malloc failed\n");
                        retval = -1;
                        goto freeall;
                }
        }

        if(clock_gettime(CLOCK_MONOTONIC_RAW, &start) == -1) {
                fprintf(stderr, "clock gettime failed\n");
                retval = -1;
                goto freeall;
        }

        //returns void
        thpool = thpool_init(number_of_threads);

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
                        fprintf(stderr, "submit job to the threadpool failed\n");
                        retval = -1;
                        goto freeall;
                }
        }

        thpool_wait(thpool);


        if(clock_gettime(CLOCK_MONOTONIC_RAW, &end) == -1) {
                fprintf(stderr, "clock gettime failed\n");
                retval = -1;
                goto freeall;
        }

        if(draw) {
                if(write_image("mandel.png", pixels, width, height) == -1) {
                        fprintf(stderr, "write image failed\n");
                        retval = -1;
                        goto freeall;
                }
        }

        retval = compute_time_milis(start, end);

freeall:
        thpool_destroy(thpool);
        for(i = 0; i < num_of_jobs; i++) {
                if(args[i] != NULL)
                        free(args[i]);
        }
freepixels:
        free(pixels);
        return retval;
}
