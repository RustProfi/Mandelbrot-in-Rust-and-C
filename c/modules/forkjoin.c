#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <pthread.h>
#include "mandel.h"

//-1.0 in case of error
double time_fork_join(int width, int height, double complex upper_left, double complex lower_right, int number_of_threads) {
        char *pixels;
        int i, offset, rows_per_band, chunk_len, arr_len;
        double retval;
        struct timespec start, end;
        pthread_t thread_id[number_of_threads];
        render_args* args[number_of_threads];

        arr_len = width * height;
        //if rows_per_band doesn't fit perfectly in arr_len without rest, it must be round upward to make sure that the bands cover the entire image.
        rows_per_band = arr_len % (height / number_of_threads) == 0 ? height / number_of_threads : height / number_of_threads + 1;
        chunk_len = rows_per_band * width;

        pixels = (char*)malloc(arr_len * sizeof(char));
        if(!pixels) {
                fprintf(stderr, "malloc failed\n");
                retval = -1;
                goto freepixels;
        }

        for(i = 0; i < number_of_threads; ++i) {
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

        i = 0;
        for(offset = 0; offset < arr_len; offset += chunk_len) {
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

                if(pthread_create(&thread_id[i], NULL, render, args[i]) != 0) {
                        fprintf(stderr, "create thread failed\n");
                        retval = -1;
                        goto freeall;
                }
                i++;
        }

        for(i = 0; i < number_of_threads; i++) {
                if(pthread_join(thread_id[i], NULL) != 0) {
                        fprintf(stderr, "join thread failed\n");
                        retval = -1;
                        goto freeall;
                };
        }

        if(clock_gettime(CLOCK_MONOTONIC_RAW, &end) == -1) {
                fprintf(stderr, "clock gettime failed\n");
                retval = -1;
                goto freeall;
        }

        if(write_image("mandel.png", pixels, width, height) == -1) {
                fprintf(stderr, "write image failed\n");
                retval = -1;
                goto freeall;
        }

        retval = compute_time_milis(start, end);

freeall:
        for(i = 0; i < number_of_threads; i++) {
                if(args[i] != NULL)
                        free(args[i]);
        }
freepixels:
        free(pixels);
        return retval;
}
