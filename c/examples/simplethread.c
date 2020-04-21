#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

void *fillarray(void *args);

int main() {
        int *arr;
        pthread_t th;

        arr = (int *)malloc(5 * sizeof(int));
        if(!arr) {
                fprintf(stderr, "malloc failed\n");
                exit(EXIT_FAILURE);
        }

        if(pthread_create(&th, NULL, fillarray, (void *)arr) != 0) {
                fprintf(stderr, "create thread failed\n");
                exit(EXIT_FAILURE);
        }

        if(pthread_join(th, NULL) != 0) {
                fprintf(stderr, "join thread failed\n");
                exit(EXIT_FAILURE);
        }
        for(int i = 0; i < 5; i++) {
                printf("%d", arr[i]);
        }
        printf("\n");
        free(arr);
}

void *fillarray(void *args) {
        int *arr = (int *)args;
        for(int i = 0; i < 5; i++) {
                arr[i] = i+1;
        }
}
