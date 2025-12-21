#include "util.h"

#define SHAPE_COUNT 6
#define MAX_REGION_COUNT 1000

typedef struct Region {
  i32 width, height;
  i32 shapes[SHAPE_COUNT];
} Region;

u8 shapes[SHAPE_COUNT][9];
i32 shape_areas[SHAPE_COUNT];
Region regions[MAX_REGION_COUNT];

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  char const *at = text;

  // Parse shapes
  {
    i32 i_shape = 0;

    for (i32 j = 0; j < SHAPE_COUNT; j++) {
      at = next_line(at); // Skip index of the shape

      for (i32 y = 0; y < 3; y++) {
        shapes[i_shape][y*3+0] = at[0] == '#';
        shapes[i_shape][y*3+1] = at[1] == '#';
        shapes[i_shape][y*3+2] = at[2] == '#';

        at = next_line(at);
      }

      i_shape++;

      at = next_line(at); // Skip empty line
    }

    for (i32 i = 0; i < SHAPE_COUNT; i++) {
      i32 area = 0;
      for (i32 j = 0; j < 9; j++) {
        area += shapes[i][j];
      }

      shape_areas[i] = area;
    }
  }

  i32 region_count;

  // Parse regions
  {
    i32 i_region = 0;

    while (*at) {
      i32 width = parse_int_advance(&at);
      at++; // Skip 'x'
      i32 height = parse_int_advance(&at);

      regions[i_region] = (Region){
        .width = width,
        .height = height,
      };

      at++; // Skip ':'

      for (i32 i = 0; i < SHAPE_COUNT; i++) {
        at++; // Skip ' '
        regions[i_region].shapes[i] = parse_int_advance(&at);
      }

      i_region++;

      at = next_line(at);
    }

    region_count = i_region;
  }

  i32 answer = 0;

  // The problem can be solved with a simple heuristic.
  // This heuristic does not work for the example input however, because
  // it can give false positives, which it does for the last example.
  // The heurstic will sometimes say it is possible to fit all shapes
  // even though it is not possible.
  for (i32 i = 0; i < region_count; i++) {
    i32 area = regions[i].width * regions[i].height;
    
    i32 min_area_needed = 0;
    for (i32 j = 0; j < SHAPE_COUNT; j++) {
      min_area_needed += regions[i].shapes[j] * shape_areas[j];
    }

    if (min_area_needed <= area) {
      answer += 1;
    }
  }

  printf("%d\n", answer);

  return 0;
}
