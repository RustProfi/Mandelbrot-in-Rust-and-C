#include <complex.h>
#include <assert.h>
#include <time.h>
#include <stdio.h>
#include "../modules/mandel.h"

void test_pixel_to_point() {
  double complex x1 = -1.0 + 1.0 * I;
  double complex x2 = 1.0 - 1.0 * I;
  double complex res = -0.5 - 0.5 * I;

  assert(pixel_to_point(100, 100, 25, 75, x1, x2) == res);
}

void test_compute_time_milis() {
  struct timespec x1 = {.tv_sec = 3, .tv_nsec = 0};
  struct timespec x2 = {.tv_sec = 4, .tv_nsec = 0};
  struct timespec x3 = {.tv_sec = 4, .tv_nsec = 500000000};
  struct timespec x4 = {.tv_sec = 6, .tv_nsec = 200000000};

  assert(compute_time_milis(x1, x2) == 1000.0);
  assert(compute_time_milis(x1, x3) == 1500.0);
  assert(compute_time_milis(x2, x3) == 500.0);
  assert(compute_time_milis(x3, x4) == 1700.0);
}

int main() {
  test_pixel_to_point();
  test_compute_time_milis();
  printf("Integration tests Success!\n");
}
