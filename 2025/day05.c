#include "util.h"

typedef struct Pair {
  i64 start, end;
} Pair;

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  i32 npairs = 0;
  Pair pairs[172];

  // Part one
  {
    i64 count = 0;

    char *at = text;
    while (is_number(*at)) {
      i64 start = parse_int(&at);
      at++; // Skip '-'
      i64 end = parse_int(&at);

      pairs[npairs++] = (Pair){ start, end + 1, };

      next_line(&at);
    }

    next_line(&at);

    while (*at) {
      i64 x = parse_int(&at);

      // naive linear search
      for (i32 i = 0; i < npairs; i++) {
        if (x >= pairs[i].start && x < pairs[i].end) {
          count++;
          break;
        }
      }

      next_line(&at);
    }

    printf("%lld\n", count);
  }

  // Part two
  {
    // Sort the pairs by start in ascending order using insertion sort
    for (i32 i = 1; i < npairs; i++) {
      for (i32 j = i; j > 0; j--) {
        if (pairs[j-1].start <= pairs[j].start) {
          break;
        }

        swap(pairs[j-1], pairs[j]);
      }
    }

    i64 count2 = 0;
    i64 at = 0;
    for (i32 i = 0; i < npairs; i++) {
      if (at < pairs[i].end) {
        count2 += pairs[i].end - max(at, pairs[i].start);
        at = pairs[i].end;
      }
    }

    printf("%lld\n", count2);
  }

  return 0;
}
