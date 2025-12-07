#include "util.h"

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);
  char *end = text;
  while (*end) { end++; }

  i32 pitch = (i32)(next_line(text) - text);

  // Assumes unix line endings
  i32 width = pitch - 1;

  i32 height = ((i32)(end - text)) / pitch;

  // Part one
  {
    text[pitch + width / 2] = '|';

    i64 count = 0;

    for (i32 y = 2; y < height; y += 2) {
      char *splitters = text + y * pitch;
      char *input = text + (y - 1) * pitch;
      char *output = text + (y + 1) * pitch;

      for (i32 x = 0; x < width; x++) {
        if (input[x] != '|') {
          continue;
        }

        if (splitters[x] == '^') {
          // There is never a splitter right at the edge, so this cannot go out of bounds
          output[x-1] = '|';
          output[x+1] = '|';
          count++;
        } else {
          splitters[x] = '|'; // drawn for a prettier picture :)
          output[x] = '|';
        }
      }
    }

    printf("%lld\n", count);
  }

  // Part two
  {
    // Note to self: ALWAYS USE 64-BIT INTEGERS!!!
    i64 input[width];
    i64 output[width];

    memset(input, 0, width * sizeof(i64));

    input[width / 2] = 1;

    for (i32 y = 2; y < height; y += 2) {
      char *splitters = text + y * pitch;

      memset(output, 0, width * sizeof(i64));

      for (i32 x = 0; x < width; x++) {
        if (splitters[x] == '^') {
          output[x-1] += input[x];
          output[x+1] += input[x];
        } else {
          output[x] += input[x];
        }
      }

      memcpy(input, output, width * sizeof(i64));
    }

    i64 sum = 0;
    for (i32 x = 0; x < width; x++) {
      sum += output[x];
    }

    printf("%lld\n", sum);
  }

  return 0;
}
