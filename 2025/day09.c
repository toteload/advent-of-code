#include "util.h"

#define MAX_POINTS 500

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  i32 points[MAX_POINTS][2];

  i32 npoints = 0;
  char *at = text;
  while (*at) {
    points[npoints][0] = parse_int_advance((const char**)&at);
    at++; // skip ','
    points[npoints][1] = parse_int_advance((const char**)&at);
    npoints++;

    assert(npoints < MAX_POINTS);

    at = next_line(at);
  }

  // Part one
  {
    // Brute force >:^)
    i64 best = 0;
    for (i32 i = 0; i < npoints - 1; i++) {
      for (i32 j = i + 1; j < npoints; j++) {
        i64 dx = 1 + abs(points[i][0] - points[j][0]);
        i64 dy = 1 + abs(points[i][1] - points[j][1]);
        i64 area = dx * dy;

        best = max(best, area);
      }
    }

    printf("%lld\n", best);
  }

  // Part two
  {
    i32 *x_sorted = malloc(sizeof(i32) * npoints);
    i32 *y_sorted = malloc(sizeof(i32) * npoints);

    i32 *x_i = malloc(sizeof(i32) * npoints);
    i32 *y_i = malloc(sizeof(i32) * npoints);

    for (i32 i = 0; i < npoints; i++) {
      x_sorted[i] = points[i][0];
      y_sorted[i] = points[i][1];

      x_i[i] = i;
      y_i[i] = i;
    }

    for (i32 i = 1; i < npoints; i++) {
      for (i32 j = i; j > 0; j--) {
        if (x_sorted[j-1] <= x_sorted[j]) {
          break;
        }

        swap(x_sorted[j-1], x_sorted[j]);
        swap(x_i[j-1], x_i[j]);
      }
    }

    for (i32 i = 1; i < npoints; i++) {
      for (i32 j = i; j > 0; j--) {
        if (y_sorted[j-1] <= y_sorted[j]) {
          break;
        }

        swap(y_sorted[j-1], y_sorted[j]);
        swap(y_i[j-1], y_i[j]);
      }
    }

    i32 *i_y = malloc(sizeof(i32) * npoints);
    i32 *i_x = malloc(sizeof(i32) * npoints);
    for (i32 i = 0; i < npoints; i++) {
      i_y[y_i[i]] = i / 2;
      i_x[x_i[i]] = i / 2;
    }

    // There are `npoints` different points, but (at most) only half that many unique
    // x and y coordinates, because each edge is either exactly horizontal or vertical.
    i32 size = npoints / 2;

    i8 *bitmap = malloc(size * size);
    memset(bitmap, 0, size * size);

    {
      // Draw all the vertical edges on the bitmap with their 'direction'.
      for (i32 i = 0; i < npoints; i++) {
        i32 j = (i + 1) % npoints;
        
        // Ignore horizontal edges
        if (points[i][1] == points[j][1]) {
          continue;
        }

        i32 dy = points[j][1] - points[i][1];
        i32 s = sign_i32(dy);
        i32 d = abs(i_y[j] - i_y[i]);

        i32 x = i_x[i];
        i32 y = i_y[i];

        for (i32 k = 0; k <= d; k++) {
          bitmap[(y + k * s) * size + x] = -s;
        }
      }

      // Fill in the polygon.
      i8 acc = 0;
      for (i32 i = 0; i < size * size; i++) {
        switch (bitmap[i]) {
          case -1:
            acc = 0;
            bitmap[i] = 1; 
            break;
          case 0:
            bitmap[i] = acc;
            break;
          case 1:
            acc = 1;
            break;
        }
      }
    }

    i64 best = 0;
    for (i32 i = 0; i < npoints - 1; i++) {
      for (i32 j = i + 1; j < npoints; j++) {
        i32 dx = 1 + abs(points[i][0] - points[j][0]);
        i32 dy = 1 + abs(points[i][1] - points[j][1]);

        i64 area = ((i64)dx) * ((i64)dy);

        i32 x_min = min(i_x[i], i_x[j]);
        i32 x_max = max(i_x[i], i_x[j]);

        i32 y_min = min(i_y[i], i_y[j]);
        i32 y_max = max(i_y[i], i_y[j]);

        i32 is_valid = 1;
        for (i32 y = y_min; y <= y_max; y++) {
          for (i32 x = x_min; x <= x_max; x++) {
            if (bitmap[y * size + x] != 1) {
              is_valid = 0;
              break;
            }
          }
        }

        if (!is_valid) {
          continue;
        }

        best = max(best, area);
      }
    }

    printf("%lld\n", best);
  }

  return 0;
}
