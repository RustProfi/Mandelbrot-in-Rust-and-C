#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <string.h>
#include "modules/mandel.h"
#include "modules/wthreads.h"
#include "modules/wthreadpool.h"

static int WIDTH = 5000;
static int HEIGHT = 5000;
static double complex UPPER_LEFT = -1.6 + 1.2 * I;
static double complex LOWER_RIGHT = 0.6 - 1.2 * I;


//run with
//gcc -o mandel main.c modules/wthreads.c modules/mandel.c modules/wthreadpool.c C-Thread-Pool/thpool.c -lm -lpng -pthread -fopenmp -Ofast
int main(int argc, char *argv[]) {

        if(argc == 1 || argc > 2) {
                fprintf(stderr, "Usage: ./mandel <arg>\n");
                fprintf(stderr, "Methods: threads|th, threadpool|tp, all\n");
                exit(EXIT_FAILURE);
        }

        if(!strcmp(argv[1], "threads") || !strcmp(argv[1], "th") || !strcmp(argv[1], "all")) {
                double res = time_fork_join(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, 12);
                if(res == -1.0) {
                        fprintf(stderr, "time fork join failed\n");
                        exit(EXIT_FAILURE);
                }
                printf("Threads: %fms\n", res);
        }

        if(!strcmp(argv[1], "threadpool") || !strcmp(argv[1], "tp") || !strcmp(argv[1], "all")) {
                double res = time_threadpool(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, 5);
                if(res == -1.0) {
                        fprintf(stderr, "threadpool failed\n");
                        exit(EXIT_FAILURE);
                }
                printf("Threadpool: %fms\n", res);
        }
}
