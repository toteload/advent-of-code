#define _CRT_SECURE_NO_WARNINGS

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

typedef int32_t i32;
typedef int64_t i64;

char* read_file(const char* path) {
  FILE* f = fopen(path, "rb");
  if (!f) {
    printf("Could not open \"%s\".\n", path);
    exit(1);
  }

  fseek(f, 0, SEEK_END);
  int32_t size = ftell(f);
  fseek(f, 0, SEEK_SET);

  char* buf = malloc(size + 1);
  if (!buf) {
    printf("malloc failed lol.\n");
    exit(1);
  }

  fread(buf, 1, size, f);
  buf[size] = '\0';
  fclose(f);

  return buf;
}

int32_t is_number(char c) {
  return c >= '0' && c <= '9';
}
