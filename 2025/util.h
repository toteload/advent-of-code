#define _CRT_SECURE_NO_WARNINGS

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

typedef int32_t i32;
typedef int64_t i64;

#define swap(a,b) do { typeof(a) _tmp = (a); a = b; b = _tmp; } while (0)

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

inline int32_t is_number(char c) {
  return c >= '0' && c <= '9';
}

inline i32 is_whitespace(char c) {
  return c == ' ' || c == '\t' || c == '\n' || c == '\r';
}

i64 parse_int_advance(char const **text) {
  char const *at = *text;

  i64 s = 0;
  while (*at && is_number(*at)) {
    s *= 10;
    s += *at - '0';
    at++;
  }

  *text = at;

  return s;
}

i64 parse_int(char const *at) {
  i64 s = 0;
  while (*at && is_number(*at)) {
    s *= 10;
    s += *at - '0';
    at++;
  }
  return s;
}

// Advances *text until it has seen a '\n'. Points to the character after '\n'.
char const * next_line(char const *at) {
  while (*at) {
    char c = *at++;
    if (c == '\n') {
      break;
    }
  }
  return at;
}

char const * until_number(char const *at) {
  while (*at && !is_number(*at)) {
    at++;
  }
  return at;
}

char const * skip_whitespace(char const *at) {
  while (*at && is_whitespace(*at)) {
    at++;
  }
  return at;
}
