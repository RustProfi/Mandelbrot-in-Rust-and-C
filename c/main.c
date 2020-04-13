#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include "modules/mandel.h"
#include "modules/forkjoin.h"

static unsigned int WIDTH = 5000;
static unsigned int HEIGHT = 5000;
static double complex UPPER_LEFT = -1.6 + 1.2 * I;
static double complex LOWER_RIGHT = 0.6 - 1.2 * I;


//run with
//gcc -o mandel main.c modules/forkjoin.c modules/mandel.c -lm -lpng -pthread -Ofast
//
int main() {
        double res = time_fork_join(WIDTH, HEIGHT, UPPER_LEFT, LOWER_RIGHT, 8);
        if(res == -1.0) {
                perror("time fork join failed");
                exit(EXIT_FAILURE);
        }
        printf("%f\n", res);
}
