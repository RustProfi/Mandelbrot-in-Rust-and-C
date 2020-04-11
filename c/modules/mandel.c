#include <stdlib.h>
#include <stdio.h>
#include <complex.h>
#include <png.h>

//Prototypes
int escape_mandel_iterations(double complex c);
double complex_norm_square(double complex z);
double complex pixel_to_point(unsigned int width, unsigned int height,
                              unsigned int p_colum, unsigned int p_row, double complex upper_left,
                              double complex lower_right);
void render(unsigned char *pixels, unsigned int width, unsigned int height,
            double complex upper_left, double complex lower_right);
int write_image(char *filename, unsigned char *pixels, unsigned int width, unsigned int height);

int escape_mandel_iterations(double complex c) {
        double complex z = 0.0 + 0.0 * I;
        for (int i = 0; i < 256; i++) {
                z = z * z + c;
                if (complex_norm_square(z) > 4.0) {
                        return i;
                }
        }
        return 0;
}

double complex_norm_square(double complex z) {
        return creal(z) * creal(z) + cimag(z) * cimag(z);
}

double complex pixel_to_point(unsigned int width, unsigned int height,
                              unsigned int p_colum, unsigned int p_row, double complex upper_left,
                              double complex lower_right) {
        double c_width = creal(lower_right) - creal(upper_left);
        double c_height = cimag(upper_left) - cimag(lower_right);

        double re = creal(upper_left) + (double) p_colum * c_width / (double) width;
        double im = cimag(upper_left) - (double) p_row * c_height / (double) height;
        return re + im * I;
}

void render(unsigned char *pixels, unsigned int width, unsigned int height,
            double complex upper_left, double complex lower_right) {
        //Ich kann hier nicht überprüfen, ob das array groß genug ist
        //Ich muss hoffen xD
        for (int row = 0; row < height; row++) {
                for (int column = 0; column < width; column++) {
                        double complex point = pixel_to_point(width, height, column, row, upper_left, lower_right);
                        pixels[row * width + column] = escape_mandel_iterations(point);
                }
        }
}

//http://www.labbookpages.co.uk/software/imgProc/files/libPNG/makePNG.c
//http://www.labbookpages.co.uk/software/imgProc/libPNG.html
//http://www.libpng.org/pub/png/libpng-1.4.0-manual.pdf
//Das normale example.c sieht ein 2 Dimensionales Array vor, was meiner Meinung
//nach alles verkompliziert.

int write_image(char *filename, unsigned char *pixels, unsigned int width, unsigned int height) {
        int code = 0;
        FILE *fp = NULL;
        png_structp png_ptr = NULL;
        png_infop info_ptr = NULL;
        png_bytep row = NULL;

        // Open file for writing (binary mode)
        fp = fopen(filename, "wb");
        if (fp == NULL) {
                fprintf(stderr, "Could not open file %s for writing\n", filename);
                code = 1;
                goto finalise;
        }

        // Initialize write structure
        png_ptr = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
        if (png_ptr == NULL) {
                fprintf(stderr, "Could not allocate write struct\n");
                code = 1;
                goto finalise;
        }

        // Initialize info structure
        info_ptr = png_create_info_struct(png_ptr);
        if (info_ptr == NULL) {
                fprintf(stderr, "Could not allocate info struct\n");
                code = 1;
                goto finalise;
        }

        // Setup Exception handling
        if (setjmp(png_jmpbuf(png_ptr))) {
                fprintf(stderr, "Error during png creation\n");
                code = 1;
                goto finalise;
        }
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
        row = (png_bytep) malloc(1 * width * sizeof(png_byte));

        // Write image data
        int x, y;
        for (y = 0; y < height; y++) {
                for (x = 0; x < width; x++) {
                        //setRGB(&(row[x*1]), pixels[y*width + x]);
                        row[x] = pixels[y * width + x];
                }
                png_write_row(png_ptr, row);
        }

        // End write
        png_write_end(png_ptr, NULL);

finalise:
        if (fp != NULL) fclose(fp);
        if (info_ptr != NULL) png_free_data(png_ptr, info_ptr, PNG_FREE_ALL, -1);
        if (png_ptr != NULL) png_destroy_write_struct( &png_ptr, (png_infopp) NULL);
        if (row != NULL) free(row);
        create header
        return code;
}

//compile with gcc -o mandel mandel.c -lm -lpng
int main() {
        //Test escape_mandel_time
        double complex z1 = -0.11456 + 0.89808 * I;
        printf("z1: %f %+f\n", creal(z1), cimag(z1));
        printf("%d\n", escape_mandel_iterations(z1)); //66

        //Test pixel_to_point
        double complex xd1 = -1.0 + 1.0 * I;
        double complex xd2 = 1.0 - 1.0 * I;
        double complex xd = pixel_to_point(100, 100, 25, 75, xd1, xd2);
        printf("z1: %f %+f\n", creal(xd), cimag(xd)); // -0.5 -0.5

}
