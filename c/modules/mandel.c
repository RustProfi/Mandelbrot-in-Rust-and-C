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

// Try to determine if c is in the Mandelbrot set, using at most 256
// iterations due to the grayscale color spectrum of the Png Writer.
//
// If c is not a member, return the number of
// iterations it took for c to leave the circle of radius two centered on the
// origin. If c seems to be a member (more precisely, if we reached the
// iteration limit without being able to prove that c is not a member),
// return 0.
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
        // Why subtraction here? p_row increases as we go down,
        // but the imaginary component increases as we go up.
        double im = cimag(upper_left) - (double) p_row * c_height / (double) height;
        return re + im * I;
}


void *render(void *arguments) {
        //cast to render_args
        render_args *args = (render_args *) arguments;
        //there is no performant proove that the array is large enough. Just hope :)
        for (int row = 0; row < args->height; row++) {
                for (int column = 0; column < args->width; column++) {
                        double complex point = pixel_to_point(args->width, args->height, column, row, args->upper_left, args->lower_right);
                        int iters = escape_mandel_iterations(point);
                        args->chunk[row * args->width + column] = iters == 0 ? 0 : 255 - iters;
                }
        }
}

void render_openmp(char *chunk, int width, int height, double complex upper_left, double complex lower_right) {
        //there is no performant proove that the array is large enough. Just hope :)
        for (int row = 0; row < height; row++) {
                for (int column = 0; column < width; column++) {
                        double complex point = pixel_to_point(width, height, column, row, upper_left, lower_right);
                        int iters = escape_mandel_iterations(point);
                        chunk[row * width + column] = iters == 0 ? 0 : 255 - iters;
                }
        }
}

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

        // Write header
        // Colortype Grayscale 8 Bit
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
        for (int r = 0; r < height; r++) {
                //copy the row at pixels[r * witdth] byte per byte in row
                for (int c = 0; c < width; c++) {
                        row[c] = pixels[r * width + c];
                }
                //write row
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
