#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <string.h>
#include "modules/mandel.h"
#include "modules/wthreads.h"
#include "modules/wthreadpool.h"
#include "modules/wopenmp.h"

static int WIDTH = 100;
static int HEIGHT = 100;
static int NTHREADS = 4;
static int DRAW = 1;
static int NOTDRAW = 0;
static double complex UPPER_LEFT = -1.6 + 1.2 * I;
static double complex LOWER_RIGHT = 0.6 - 1.2 * I;


//run with
//gcc -o mandel main.c modules/wthreads.c modules/mandel.c modules/wthreadpool.c modules/wopenmp.c C-Thread-Pool/thpool.c -lm -lpng -pthread -fopenmp -Ofast
int main(int argc, char *argv[]) {

        if(argc == 1 || argc > 3) {
                fprintf(stderr, "Usage: mandelbrot <Method> [args]\n");
                fprintf(stderr, "Methods: threads|th, threadpool|tp, openmp|op, all\n");
                fprintf(stderr, "args: -m (Performance measure)\n");
                exit(EXIT_FAILURE);
        }

        //Performance check for threads
        //Runs 20x for each thread_count.
        if(!strcmp(argv[1], "threads") || !strcmp(argv[1], "th") || !strcmp(argv[1], "all")) {
                if(argv[2] != NULL && !strcmp(argv[2], "-m")) {
                        FILE *fp;
                        fp = fopen("c_threads_performance.txt", "w");
                        if (!fp) {
                                perror("Could not open file for writing");
                                exit(EXIT_FAILURE);
                        }

                        for(int thread_count = 4; thread_count <= 60; thread_count++) {
                                double time = 0;
                                for(int i = 0; i < 20; i++) {
                                        double res = time_threads(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, thread_count, NOTDRAW);
                                        if(res == -1.0) {
                                                perror("time with threads failed");
                                                exit(EXIT_FAILURE);
                                        }
                                        time += res;
                                }
                                time /= 20;
                                fprintf(fp, "%d,%f\n", thread_count, time);
                        }
                        fclose(fp);
                }
                else {
                        double res = time_threads(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, NTHREADS, DRAW);
                        if(res == -1.0) {
                                perror("time with threads failed");
                                exit(EXIT_FAILURE);
                        }
                        printf("Time with threading: %.2fms\n", res);
                }

        }

        //Performance check for threadpool
        //Runs 20x for each rows_per_band.
        //Threadpool always with 8 threads.
        if(!strcmp(argv[1], "threadpool") || !strcmp(argv[1], "tp") || !strcmp(argv[1], "all")) {
                if(argv[2] != NULL && !strcmp(argv[2], "-m")) {
                        FILE *fp;
                        fp = fopen("c_threadpool_performance.txt", "w");
                        if (!fp) {
                                perror("Could not open file for writing");
                                exit(EXIT_FAILURE);
                        }

                        for(int rows_per_band = 1; rows_per_band <= 60; rows_per_band++) {
                                double time = 0;
                                for(int i = 0; i < 20; i++) {
                                        double res = time_threadpool(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, rows_per_band, 8, NOTDRAW);
                                        if(res == -1.0) {
                                                perror("time with threadpool failed");
                                                exit(EXIT_FAILURE);
                                        }
                                        time += res;
                                }
                                time /= 20;
                                fprintf(fp, "%d,%f\n", rows_per_band, time);
                        }
                        fclose(fp);
                }
                else {
                        double res = time_threadpool(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, 5, 8, DRAW);
                        if(res == -1.0) {
                                perror("time with threadpool failed");
                                exit(EXIT_FAILURE);
                        }
                        printf("Time with threadpool: %.2fms\n", res);
                }

        }

        //Performance check for openmp
        //Runs 20x for each thread_count.
        if(!strcmp(argv[1], "openmp") || !strcmp(argv[1], "op") || !strcmp(argv[1], "all")) {
                if(argv[2] != NULL && !strcmp(argv[2], "-m")) {
                        FILE *fp;
                        fp = fopen("c_openmp_performance.txt", "w");
                        if (!fp) {
                                perror("Could not open file for writing");
                                exit(EXIT_FAILURE);
                        }

                        for(int thread_count = 4; thread_count <= 60; thread_count++) {
                                double time = 0;
                                for(int i = 0; i < 20; i++) {
                                        double res = time_openmp(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, 5, thread_count, NOTDRAW);
                                        if(res == -1.0) {
                                                perror("time with openmp failed");
                                                exit(EXIT_FAILURE);
                                        }
                                        time += res;
                                }
                                time /= 20;
                                fprintf(fp, "%d,%f\n", thread_count, time);
                        }
                        fclose(fp);
                }
                else {
                        double res = time_openmp(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, 5, NTHREADS, DRAW);
                        if(res == -1.0) {
                                perror("time with openmp failed");
                                exit(EXIT_FAILURE);
                        }
                        printf("Time with openmp: %.2fms\n", res);
                }
        }
}
