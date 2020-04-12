#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <pthread.h>
#include "mandel.h"

//0 in case of error
//todo noch alle mallocs immer freen Xd
double time_fork_join(unsigned int width, unsigned int height, double complex upper_left, double complex lower_right, unsigned int number_of_threads) {
        unsigned char *pixels;
        int i, offset, rows_per_band, chunk_len, arr_len;
        struct timespec start, end;
        pthread_t thread_id[number_of_threads];
        render_args* args[number_of_threads];

        arr_len = width * height;
        rows_per_band = height / number_of_threads + 1;
        chunk_len = rows_per_band * width;

        pixels = (unsigned char*)malloc(width * height * sizeof(unsigned char));

        if(!pixels) {
                //free(thread_id);
                perror("malloc failed");
                exit(EXIT_FAILURE);
        }

        for(i = 0; i < number_of_threads; ++i) {
                args[i] = (render_args*)malloc(sizeof(render_args));
                if(!args[i]) {
                  perror("malloc failed");
                  exit(EXIT_FAILURE);
                }
        }

        if(clock_gettime(CLOCK_MONOTONIC_RAW, &start) == -1) {
                perror("clock gettime");
                exit(EXIT_FAILURE);
        }

        i = 0;
        for(offset = 0; offset < arr_len; offset += chunk_len) {
                unsigned int check_chunk_len = arr_len - offset > chunk_len ? chunk_len : arr_len - offset;
                unsigned int top = rows_per_band * i;
                //unsigned int band_width = width;
                unsigned int band_height = check_chunk_len / width;
                double complex band_upper_left = pixel_to_point(width, height, 0, top, upper_left, lower_right);
                double complex band_lower_right = pixel_to_point(width, height, width, top + band_height, upper_left, lower_right);

                args[i]->pixels = &pixels[offset];
                args[i]->width = width;
                args[i]->height = band_height;
                //printf("height %d\n", args[i]->height);
                args[i]->upper_left = band_upper_left;
                args[i]->lower_right = band_lower_right;

                int res = pthread_create(&thread_id[i], NULL, render, args[i]);

                if(res != 0) {
                        perror("create thread failed");
                        free(pixels);
                        //free(thread_id);
                        exit(EXIT_FAILURE);
                }
                i++;
        }

        for(i = 0; i < number_of_threads; i++) {
                pthread_join(thread_id[i], NULL);
        }


        if(clock_gettime(CLOCK_MONOTONIC_RAW, &end) == -1) {
                perror("clock gettime failed");
                exit(EXIT_FAILURE);
        }

        //write_image
        int r = write_image("mandel.png", pixels, width, height);
        if(r != 0) {
                perror("write image failed");
                free(pixels);
                //free(thread_id);
                exit(EXIT_FAILURE);
        }

        free(pixels);
        for(i = 0; i < number_of_threads; i++) {
          free(args[i]);
        }
        return compute_time_milis(start, end);

}
