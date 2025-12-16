#include "util.h"

#define MAX_BOXES 1000
#define MAX_CONNECTIONS 1000

typedef struct Pair {
  i32 idx[2];
} Pair;

typedef struct Box {
  i32 data[3];
} Box;

i64 box_distance(Box *a, Box *b) {
  i64 dx = a->data[0] - b->data[0];
  i64 dy = a->data[1] - b->data[1];
  i64 dz = a->data[2] - b->data[2];
  return dx*dx + dy*dy + dz*dz;
}

Pair *find_insert(Box *box, Pair *start, Pair *end, i64 d) {
  if (start == end) {
    return start;
  }

  i32 len = (i32)(end - start);
  Pair *mid = start + len / 2;

  i64 d_mid = box_distance(box + mid->idx[0], box + mid->idx[1]);

  if (d < d_mid) {
    return find_insert(box, start, mid, d);
  }

  return find_insert(box, mid + 1, end, d);
}

i32 flood(Pair *pairs, Pair **visited) {
  if (pairs == *visited) {
    return 0;
  }

  *visited = *visited - 1;

  Pair curr = *pairs;

  swap(*pairs, **visited);

  i32 sum = 0;

  Pair *at = pairs;
  while (at < *visited) {
    if (at->idx[0] == curr.idx[0] ||
        at->idx[0] == curr.idx[1] ||
        at->idx[1] == curr.idx[0] ||
        at->idx[1] == curr.idx[1])
    {
      sum += flood(pairs, visited);
    }

    at++;
  }

  return 1 + sum;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 3) {
    printf("Usage: %s input.txt <pair-count>\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);
  i32 npairs = parse_int(argv[2]);

  Box box[MAX_BOXES];

  i32 len = 0;
  char const *at = text;
  while (*at) {
    i32 x = parse_int_advance(&at);
    at++; // skip ','
    i32 y = parse_int_advance(&at);
    at++; // skip ','
    i32 z = parse_int_advance(&at);

    box[len++] = (Box){ .data = { x, y, z, }, };

    at = next_line(at);
  }

  Pair pairs[MAX_CONNECTIONS];
  for (i32 i = 0, k = 0; i < len - 1; i++) {
    for (i32 j = i + 1; j < len; j++, k++) {
      i64 d = box_distance(box + i, box + j);

      Pair *end = pairs + min(k, npairs);
      Pair *insert = find_insert(box, pairs, end, d);

      if (insert == end && i >= npairs) {
        continue;
      }

      end = pairs + min(k+1, npairs);
      memmove(insert + 1, insert, sizeof(Pair) * (end - insert));

      *insert = (Pair){ .idx = { i, j, }, };
    }
  }

  for (i32 i = 0; i < npairs; i++) {
    i64 d = box_distance(box + pairs[i].idx[0], box + pairs[i].idx[1]);
    printf("distance %lld, %d %d\n", d, pairs[i].idx[0], pairs[i].idx[1]);
  }

  i32 best[3] = { 0, 0, 0, };

  Pair *visited = pairs + npairs;
  while (visited != pairs) {
    i32 size = flood(pairs, &visited);

    printf("%d\n", size);

    if (size > best[0]) { best[0] = size; }

    if (best[0] > best[1]) { swap(best[0], best[1]); }
    if (best[1] > best[2]) { swap(best[1], best[2]); }
  }

  i64 answer = best[0] * best[1] * best[2];
  printf("%lld\n", answer);

  return 0;
}
