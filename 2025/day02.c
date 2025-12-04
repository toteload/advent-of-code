#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

char* read_file(const char* path) {
  FILE* f = fopen(path, "rb");
  if (!f) {
    printf("Could not open file.\n");
    exit(1);
  }

  fseek(f, 0, SEEK_END);
  int32_t size = ftell(f);
  fseek(f, 0, SEEK_SET);

  char* buf = malloc(size + 1);
  if (!buf) {
    printf("Memory error.\n");
    exit(1);
  }

  fread(buf, 1, size, f);
  buf[size] = '\0';
  fclose(f);

  return buf;
}

int32_t is_invalid(int64_t x) {
  char buf[64];
  int32_t len = 0;

  int64_t tmp = x;
  while (tmp) {
    buf[len++] = tmp % 10;
    tmp /= 10;
  }

  if (len % 2 == 1) {
    return 0;
  }

  for (int32_t i = 0; i < len / 2; i++) {
    if (buf[i] != buf[i + len / 2]) {
      return 0;
    }
  }

  return 1;
}

int32_t is_invalid2(int64_t x) {
  char buf[64];
  int32_t len = 0;

  int64_t tmp = x;
  while (tmp) {
    buf[len++] = tmp % 10;
    tmp /= 10;
  }

  for (int32_t i = 1; i <= len / 2; i++) {
    if (len % i != 0) {
      continue;
    }

    int32_t parts = len / i;
    for (int32_t j = 1; j < parts; j++) {
      for (int32_t k = 0; k < i; k++) {
        if (buf[(j-1)*i+k] != buf[j*i+k]) {
          goto next;
        }
      }
    }

    return 1;
next:
    continue;
  }

  return 0;

}

int32_t main(int32_t argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  int64_t sum = 0;
  int64_t sum2 = 0;

  char *at = text;
  while (*at) {
    int64_t start = 0;
    while (*at >= '0' && *at <= '9') {
      start *= 10;
      start += *at++ - '0';
    }

    // Skip the '-'
    at++;

    int64_t end = 0;
    while (*at >= '0' && *at <= '9') {
      end *= 10;
      end += *at++ - '0';
    }

    for (int64_t x = start; x <= end; x++) {
      if (is_invalid(x)) {
        sum += x;
      }

      if (is_invalid2(x)) {
        sum2 += x;
      }
    }

    while (*at && !(*at >= '0' && *at <= '9')) {
      at++;
    }
  }

  printf("%lld\n%lld\n", sum, sum2);

  return 0;
}
