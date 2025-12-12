#include "util.h"

#define MAX_BUTTONS 16
#define MAX_LINES 180

u16 lights[MAX_LINES];
u16 buttons[MAX_LINES][MAX_BUTTONS];
u8 button_counts[MAX_LINES];

char *parse_light(char *at, u16 *light) {
  at++; // Skip '['

  u16 acc = 0;
  i32 i = 0;
  while (*at != ']') {
    acc |= (*at == '#') << i;
    at++;
    i++;
  }

  at++; // Skip ']'

  *light = acc;

  return at;
}

char *parse_button(char *at, u16 *button) {
  at++; // Skip '('

  u16 acc = 0;
  while (*at != ')') {
    i64 idx;
    at = parse_i64(at, &idx);
    acc |= 1 << idx;

    if (*at == ',') {
      at++;
    }
  }

  at++; // Skip ')'

  *button = acc;

  return at;
}

i64 find_min_press_count(u16 light, u16 const *buttons, u8 button_count) {
  i64 best = button_count;

  for (i32 select = 0; select < (1 << button_count); select++) {
    u16 acc = 0;
    for (i32 j = 0; j < button_count; j++) {
      if (select & (1 << j)) {
        acc ^= buttons[j];
      }
    }

    if (acc == light) {
      best = min(best, __builtin_popcount(select));
    }
  }

  return best;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  char *at = text;
  i32 len = 0;
  while (*at) {
    at = parse_light(at, &lights[len]);
    assert(*at == ' ');
    at++; // Skip ' '
    
    i32 button_count = 0;
    while (*at == '(') {
      at = parse_button(at, &buttons[len][button_count]);
      at++; // Skip ' '
      button_count++;
    }

    button_counts[len] = button_count;
    len++;

    // Skip the joltage requirements
    at = next_line(at);
  }

  i64 sum = 0;
  for (i32 i = 0; i < len; i++) {
    sum += find_min_press_count(lights[i], buttons[i], button_counts[i]);
  }

  printf("%lld\n", sum);

  return 0;
}
