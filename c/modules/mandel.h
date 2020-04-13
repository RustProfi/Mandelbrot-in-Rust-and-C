#ifndef MANDEL_H_   /* guard */
#define MANDEL_H_

double complex pixel_to_point(int width, int height,
                              int p_colum, int p_row, double complex upper_left,
                              double complex lower_right);
void *render(void *args);
int write_image(char *filename, char *pixels, int width, int height);
double compute_time_milis(struct timespec start, struct timespec end);

typedef struct {
        char *chunk;
        int width;
        int height;
        double complex upper_left;
        double complex lower_right;
} render_args;

#endif // MANDEL_H_
