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

i64 joltage(char const *buf, i32 len, i32 ndigits) {
  i64 acc = 0;

  i32 lo = 0;
  for (i32 i = 0; i < ndigits; i++) {
    i32 idx = find_max_idx(buf, lo, len - (ndigits - 1 - i));
    acc = (10 * acc) + buf[idx];
    lo = idx + 1;
  }

  return acc;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  i64 sum = 0;
  i64 sum2 = 0;

  char buf[100];
  char *at = text;
  while (*at) {
    i32 len = 0; 
    while (is_number(*at)) {
      buf[len++] = *at++ - '0';
    }

    sum  += joltage(buf, len, 2);
    sum2 += joltage(buf, len, 12);

    while (*at && !is_number(*at)) {
      at++;
    }
  }

  printf("%lld\n%lld\n", sum, sum2);

  return 0;
}
