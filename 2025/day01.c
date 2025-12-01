#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int32_t main() {
  FILE *f = fopen("input.txt", "rb");
  if (!f) {
    return 1;
  }
  fseek(f, 0, SEEK_END);
  int32_t size = ftell(f);
  fseek(f, 0, SEEK_SET);
  char *text = malloc(size + 1);
  fread(text, size, 1, f);
  text[size] = '\0';

  int32_t count = 0;
  int32_t pcount = 0;
  int32_t dial = 50;

  char *at = text;
  while (*at) {
    int32_t sign = (*at++ == 'L') ? -1 : 1;

    int32_t n = *at++ - '0';
    while (*at >= '0' && *at <= '9') {
      n *= 10;
      n += *at++ - '0';
    }

    pcount += n / 100;

    n %= 100;

    int32_t was_on_zero = dial == 0;
    dial += sign * n;
    if (dial < 0)    { dial += 100; pcount += !was_on_zero; }
    if (dial >= 100) { dial -= 100; pcount++; }

    if (dial == 0) {
      count++;

      if (sign < 0) {
        pcount++;
      }
    }

    while (*at) {
      if (*at == 'L' || *at == 'R') {
        break;
      }
      at++;
    }
  }

  printf("%d\n", count);
  printf("%d\n", pcount);

  return 0;
}
