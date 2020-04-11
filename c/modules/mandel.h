#ifndef MANDEL_H_   /* guard */
#define MANDEL_H_

double complex pixel_to_point(unsigned int width, unsigned int height,
                              unsigned int p_colum, unsigned int p_row, double complex upper_left,
                              double complex lower_right);
void render(unsigned char *pixels, unsigned int width, unsigned int height,
            double complex upper_left, double complex lower_right);
int write_image(char *filename, unsigned char *pixels, unsigned int width, unsigned int height);

#endif // MANDEL_H_
