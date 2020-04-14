default:
	gcc -o mandel main.c modules/wthreads.c modules/mandel.c modules/wthreadpool.c modules/wopenmp.c C-Thread-Pool/thpool.c -lm -lpng -pthread -fopenmp -Ofast
clean:
	rm mandel mandel.png
