#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <time.h>
#include <png.h>
#include "mandel.h"

//Prototypes
int escape_mandel_iterations(double complex c);
double complex pixel_to_point( int width, int height,
                               int p_colum, int p_row, double complex upper_left,
                               double complex lower_right);
void *render(void *args);
void render_openmp(char *chunk, int width, int height, double complex upper_left, double complex lower_right);
int write_image(char *filename, char *pixels, int width, int height);
double compute_time_milis(struct timespec start, struct timespec end);

int escape_mandel_iterations(double complex c) {
        double complex z = 0.0 + 0.0 * I;
        for (int i = 0; i < 256; i++) {
                z = z * z + c;
                if (cabs(z) > 4.0) {
                        return i;
                }
        }
        return 0;
}

double complex pixel_to_point(int width, int height,
                               int p_colum, int p_row, double complex upper_left,
                               double complex lower_right) {
        double c_width = creal(lower_right) - creal(upper_left);
        double c_height = cimag(upper_left) - cimag(lower_right);

        double re = creal(upper_left) + (double) p_colum * c_width / (double) width;
        double im = cimag(upper_left) - (double) p_row * c_height / (double) height;
        return re + im * I;
}


void *render(void *arguments) {
        //Ich kann hier nicht überprüfen, ob das array groß genug ist
        //Ich muss hoffen xD
        render_args *args = (render_args *) arguments;
        for (int row = 0; row < args->height; row++) {
                for (int column = 0; column < args->width; column++) {
                        double complex point = pixel_to_point(args->width, args->height, column, row, args->upper_left, args->lower_right);
                        int iters = escape_mandel_iterations(point);
                        args->chunk[row * args->width + column] = iters == 0 ? 0 : 255 - iters;
                }
        }
}

void render_openmp(char *chunk, int width, int height, double complex upper_left, double complex lower_right) {
        //Ich kann hier nicht überprüfen, ob das array groß genug ist
        //Ich muss hoffen xD
        for (int row = 0; row < height; row++) {
                for (int column = 0; column < width; column++) {
                        double complex point = pixel_to_point(width, height, column, row, upper_left, lower_right);
                        int iters = escape_mandel_iterations(point);
                        chunk[row * width + column] = iters == 0 ? 0 : 255 - iters;
                }
        }
}

//http://www.labbookpages.co.uk/software/imgProc/files/libPNG/makePNG.c
//http://www.labbookpages.co.uk/software/imgProc/libPNG.html
//http://www.libpng.org/pub/png/libpng-1.4.0-manual.pdf
//Das normale example.c sieht ein 2 Dimensionales Array vor, was meiner Meinung
//nach alles verkompliziert.

//in case of return 0 everything is ok
int write_image(char *filename, char *pixels, int width, int height) {
        int code = 0;
        FILE *fp;
        png_structp png_ptr;
        png_infop info_ptr;
        png_bytep row;

        // Open file for writing (binary mode)
        fp = fopen(filename, "wb");
        if (fp == NULL) {
                fprintf(stderr, "Could not open file %s for writing\n", filename);
                code = -1;
                goto finalise;
        }

        // Initialize write structure
        png_ptr = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
        if (png_ptr == NULL) {
                fprintf(stderr, "Could not allocate write struct\n");
                code = -1;
                goto finalise;
        }

        // Initialize info structure
        info_ptr = png_create_info_struct(png_ptr);
        if (info_ptr == NULL) {
                fprintf(stderr, "Could not allocate info struct\n");
                code = -1;
                goto finalise;
        }

        // Setup Exception handling
        if (setjmp(png_jmpbuf(png_ptr))) {
                fprintf(stderr, "Error during png creation\n");
                code = -1;
                goto finalise;
        }

        png_init_io(png_ptr, fp);

        // Write header (8 bit colour depth)
        png_set_IHDR(
                png_ptr,
                info_ptr,
                width, height,
                8,
                PNG_COLOR_TYPE_GRAY,
                PNG_INTERLACE_NONE,
                PNG_COMPRESSION_TYPE_DEFAULT,
                PNG_FILTER_TYPE_DEFAULT
                );

        //write settings
        png_write_info(png_ptr, info_ptr);

        // Allocate memory for one row (1 bytes per pixel - Grayscale)
        row = (png_bytep)malloc(1 * width * sizeof(png_byte));

        // Write image data
        for (int y = 0; y < height; y++) {
                for (int x = 0; x < width; x++) {
                        row[x] = pixels[y * width + x];
                }

                png_write_row(png_ptr, row);
        }

        // End write
        png_write_end(png_ptr, NULL);

finalise:
        if (fp) fclose(fp);
        if (info_ptr) png_free_data(png_ptr, info_ptr, PNG_FREE_ALL, -1);
        if (png_ptr) png_destroy_write_struct(&png_ptr, &info_ptr);
        if (row) free(row);

        return code;
}

double compute_time_milis(struct timespec start, struct timespec end) {
        return (end.tv_sec - start.tv_sec) * 1000.0
               + (end.tv_nsec - start.tv_nsec) / 1000000.0;
}

//compile with gcc -o mandel mandel.c -lm -lpng
/*
   int main() {
        //Test escape_mandel_iterations
        double complex z1 = -0.11456 + 0.89808 * I;
        printf("z1: %f %+f\n", creal(z1), cimag(z1));
        printf("%d\n", escape_mandel_iterations(z1)); //66

        //Test pixel_to_point
        double complex xd1 = -1.0 + 1.0 * I;
        double complex xd2 = 1.0 - 1.0 * I;
        double complex xd = pixel_to_point(100, 100, 25, 75, xd1, xd2);
        printf("z1: %f %+f\n", creal(xd), cimag(xd)); // -0.5 -0.5

   }*/
