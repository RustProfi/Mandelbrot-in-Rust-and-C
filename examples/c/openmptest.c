#include <stdio.h>
#include <stdlib.h>
#include <omp.h>
void xd(int* x, int i) {
  x[0] = i;
}


int main() {
  int *buf = (int*)malloc(8*sizeof(int));
  #pragma omp parallel for default(none) shared(buf)
  for (int i = 0; i < 8; i++) {
    xd(&buf[i], i);
  }
  //printf("%d\n", i);
  for(int j = 0; j < 8; ++j) {
    printf("%d\n", buf[j]);
  }
  free(buf);
  return 0;
}
