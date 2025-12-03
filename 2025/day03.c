#include "util.h"

i32 find_max_idx(char const *xs, i32 lo, i32 hi) {
  char s = 0;
  i32 idx = lo;
  for (i32 i = lo; i < hi; i++) {
    if (xs[i] > s) {
      s = xs[i];
      idx = i;
    }
  }
  return idx;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  // Part one
  {
    i64 sum = 0;
    char buf[100];
    char *at = text;
    while (*at) {
      i32 len = 0; 
      while (is_number(*at)) {
        buf[len++] = *at++ - '0';
      }

      char best = 0;
      for (i32 i = 1; i < len; i++) {
        char lmax = buf[find_max_idx(buf, 0, i)];
        char rmax = buf[find_max_idx(buf, i, len)];
        best = max(best, lmax * 10 + rmax);
      }

      sum += best;

      while (*at && !is_number(*at)) {
        at++;
      }
    }

    printf("%lld\n", sum);
  }

  // Part two
  {
    i64 sum = 0;
    char buf[100];
    char *at = text;
    while (*at) {
      i32 len = 0; 
      while (is_number(*at)) {
        buf[len++] = *at++ - '0';
      }

      i64 acc = 0;

      i32 lo = 0;
      for (i32 i = 0; i < 12; i++) {
        i32 idx = find_max_idx(buf, lo, len - (11 - i));
        acc = (10 * acc) + buf[idx];
        lo = idx + 1;
      }

      sum += acc;

      while (*at && !is_number(*at)) {
        at++;
      }
    }

    printf("%lld\n", sum);
  }

  return 0;
}
