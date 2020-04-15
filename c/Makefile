default:
	gcc -o mandelbrot main.c modules/wthreads.c modules/mandel.c modules/wthreadpool.c modules/wopenmp.c C-Thread-Pool/thpool.c -lm -lpng -pthread -fopenmp -Ofast
clean:
	rm mandelbrot mandel.png
