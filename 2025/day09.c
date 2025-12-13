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
    // ..............
    // .......0---1..
    // .......|...|..
    // ..6----7...|..
    // ..|........|..
    // ..5------4.|..
    // .........|.|..
    // .........3-2..
    // ..............

    // 1. go over all pairs
    //   - if other point is on wrong side of line -> skip
    //   - go over all negative areas
    //     - if any overlap with current area -> invalid area

    i64 best = 0;
    for (i32 i = 0; i < npoints - 1; i++) {
      // The -1 and 1 values for the sign are dependent on the winding of the shape,
      // but because we only need to solve one instance they are hardcoded.
      // The winding is the same for the example and input (at least for me).
      i32 dim;
      i32 sign;
      if (points[i][0] == points[i+1][0]) {
        dim = 0;
        sign = (points[i][1] < points[i+1][1]) ? -1 : 1;
      } else {
        dim = 1;
        sign = (points[i][0] < points[i+1][0]) ? 1 : -1;
      }

      //printf("dim=%d for %d\n", dim, i);

      for (i32 j = i + 1; j < npoints; j++) {
        i32 x_i = points[i][0];
        i32 x_j = points[j][0];

        i32 y_i = points[i][1];
        i32 y_j = points[j][1];

        if (sign * points[j][dim] < sign * points[i][dim]) {
          //printf("%d on outside for (%d, %d), because %d < %d\n", j, i, j, sign * points[j][dim], sign * points[i][dim]);
          continue;
        }

        i64 dx = 1 + abs(x_i - x_j);
        i64 dy = 1 + abs(y_i - y_j);

        i64 area = dx * dy;
      }
    }

#if 0
    i64 best = 0;
    for (i32 i = 0; i < npoints - 1; i++) {
      i32 dim;
      if (points[i][0] == points[i+1][0]) {
        dim = 1;
      } else {
        dim = 0;
      }

      for (i32 j = i + 1; j < npoints; j++) {
        i32 x_i = points[i][0];
        i32 x_j = points[j][0];

        i32 y_i = points[i][1];
        i32 y_j = points[j][1];

        if (points[j][dim] < points[i][dim]) {
          continue;
        }

        i64 dx = 1 + abs(x_i - x_j);
        i64 dy = 1 + abs(y_i - y_j);

        i64 area = dx * dy;

        //printf("%lld\n", area);

        if (area <= best) {
          continue;
        }

        i32 is_valid = 1;
        for (i32 k = 0; k < npoints; k++) {
          if (k == i || k == j) {
            continue;
          }

          i32 x_min = min(x_i, x_j);
          i32 x_max = max(x_i, x_j);

          i32 y_min = min(y_i, y_j);
          i32 y_max = max(y_i, y_j);

          i32 x_k = points[k][0];
          i32 y_k = points[k][1];

          if (x_k > x_min && x_k < x_max && y_k > y_min && y_k < y_max) {
            is_valid = 0;
            break;
          }
        }

        if (!is_valid) {
          continue;
        }

        printf("new best! %lld from %d to %d\n", area, i, j);

        best = area;
      }
    }
#endif

    // 4531758980 too high
    // 3252683184 too high
    printf("%lld\n", best);
  }

  return 0;
}
