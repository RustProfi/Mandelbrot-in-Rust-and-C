#include <stdio.h>

#include <complex.h>

double complex_norm_square(double complex z);
double complex pixel_to_point(unsigned int width, unsigned int height,
  unsigned int p_colum, unsigned int p_row, double complex upper_left,
  double complex lower_right);

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

void render(unsigned char* pixels, unsigned int width, unsigned int height,
  double complex upper_left, double complex lower_right) {
    //Ich kann hier nicht überprüfen, ob das array groß genug ist
    //Ich muss hoffen xD
    for(int row = 0; row < height; row++) {
      for(int column = 0; column < width; column++) {
        double complex point = pixel_to_point(width, height, column, row, upper_left, lower_right);
        pixels[row * width + column] = escape_mandel_iterations(point);
      }
    }
  }

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
