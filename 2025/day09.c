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

  // Brute force >:)
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

  {
    i32 minx = INT32_MAX, maxx = INT32_MIN;
    i32 miny = INT32_MAX, maxy = INT32_MIN;

    for (i32 i = 0; i < npoints; i++) {
      minx = min(minx, points[i][0]);
      maxx = max(maxx, points[i][0]);
      miny = min(miny, points[i][0]);
      maxy = max(maxy, points[i][0]);
    }

    printf("%dx%d - %dx%d\n", minx, miny, maxx, maxy);
  }

  return 0;
}
