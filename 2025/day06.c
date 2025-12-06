#include "util.h"
#include <string.h>
#include <assert.h>

i32 main(i32 argc, char const **argv) {
  if (argc < 4) {
    printf("Usage: %s input.txt <width> <height>\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  i32 width = parse_int(argv[2]);
  i32 height = parse_int(argv[3]);

  // Part one
  {
    i64 numbers[width * height];
    char ops[width];

    char const *at = text;
    for (i32 i = 0; i < width * height; i++) {
      at = until_number(at);
      numbers[i] = parse_int_advance(&at); 
    }

    at = next_line(at);

    for (i32 i = 0; i < width; i++) {
      at = skip_whitespace(at);
      ops[i] = *at;
      at++;
    }

    i64 sum = 0;
    for (i32 x = 0; x < width; x++) {
      i64 acc = numbers[x];
      for (i32 y = 1; y < height; y++) {
        i64 z = numbers[y * width + x];

        if (ops[x] == '*') {
          acc *= z;
        }

        if (ops[x] == '+') {
          acc += z;
        }
      }

      sum += acc;
    }

    printf("%lld\n", sum);
  }

  // Part two
  {
    // In the input, each line has the same number of bytes.
    i32 pitch = (i32)(next_line(text) - text);

    char const *ops = text + height * pitch;

    i64 sum = 0;

    char const *at = ops;
    for (i32 x = 0; x < width; x++) {
      char const *next = skip_whitespace(at + 1);

      // number of entries in this column
      i32 d = (i32)(next - at) - 1;

      assert(d <= 4);
      i64 buf[4] = { 0 };

      char const *col = at - height * pitch;

      // store each vertical number in this entry into a buffer
      for (i32 i = 0; i < d; i++) {

        // find the start of the number
        char const *start = col + i;
        for (i32 j = 0; j < height; j++) {
          if (is_number(start[j * pitch])) {
            start += j * pitch;
            break;
          }
        }

        // find the end of the number
        char const *end = start;
        while (end != (at + i) && is_number(*end)) {
          end += pitch;
        }

        // read the number
        i64 acc = 0;
        while (start != end) {
          acc *= 10;
          acc += *start - '0';
          start += pitch;
        }

        buf[i] = acc;
      }

      i64 acc = buf[0];
      for (i32 i = 1; i < d; i++) {
        if (*at == '*') {
          acc *= buf[i];
        }

        if (*at == '+') {
          acc += buf[i];
        }
      }

      sum += acc;

      at = next;
    }

    printf("%lld\n", sum);
  }

  return 0;
}
