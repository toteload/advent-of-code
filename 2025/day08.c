#include "util.h"

#define MAX_BOXES 1000
#define MAX_CONNECTIONS 1000
#define MAX_DISTANCE_COUNT ((MAX_CONNECTIONS * (MAX_CONNECTIONS - 1)) / 2)

typedef struct Pair {
  i32 idx[2];
} Pair;

typedef struct Box {
  i32 data[3];
} Box;

Box boxes[MAX_BOXES];
i64 distances[MAX_DISTANCE_COUNT];
Pair pairs[MAX_DISTANCE_COUNT];

i32 has_common_point(Pair a, Pair b) {
  return a.idx[0] == b.idx[0] || 
         a.idx[0] == b.idx[1] ||
         a.idx[1] == b.idx[0] ||
         a.idx[1] == b.idx[1];
}

inline i64 box_distance(Box a, Box b) {
  i64 dx = a.data[0] - b.data[0];
  i64 dy = a.data[1] - b.data[1];
  i64 dz = a.data[2] - b.data[2];
  return dx*dx + dy*dy + dz*dz;
}

i32 find_insert(i64 *distances, i32 start, i32 end, i64 d) {
  if (start == end) {
    return start;
  }

  i32 mid = start + (end - start) / 2;

  if (d < distances[mid]) {
    return find_insert(distances, start, mid, d);
  }

  return find_insert(distances, mid + 1, end, d);
}

i32 flood(Pair *pairs, i32 start, i32 *visited) {
  i32 end = *visited - 1;

  swap(pairs[start], pairs[end]);

  i32 at = end;
  while (at >= end) {
    for (i32 i = 0; i < end;) {
      if (has_common_point(pairs[i], pairs[at])) {
        end--;
        swap(pairs[i], pairs[end]);
        continue;
      }

      i++;
    }

    at--;
  }

  i32 count = *visited - end;

  *visited = end;

  return count;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 3) {
    printf("Usage: %s input.txt <pair-count>\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);
  i32 npairs = parse_int(argv[2]);

  i32 box_count = 0;
  char const *at = text;
  while (*at) {
    i32 x = parse_int_advance(&at);
    at++; // skip ','
    i32 y = parse_int_advance(&at);
    at++; // skip ','
    i32 z = parse_int_advance(&at);

    boxes[box_count++] = (Box){ .data = { x, y, z, }, };

    at = next_line(at);
  }

  i32 distance_count = (box_count * (box_count - 1)) / 2;

  // This loop is a LOT slower than I expected.
  for (i32 i = 0, k = 0; i < box_count - 1; i++) {
    for (i32 j = i + 1; j < box_count; j++, k++) {
      i64 d = box_distance(boxes[i], boxes[j]);

      i32 insert = find_insert(distances, 0, k, d);

      if (insert != k) {
        memmove(distances + insert + 1, distances + insert, sizeof(i64) * (k - insert));
        memmove(pairs + insert + 1, pairs + insert, sizeof(Pair) * (k - insert));
      }

      distances[insert] = d;
      pairs[insert] = (Pair){ .idx = { i, j, }, };
    }
  }

  // Part two
  i64 answer2 = 0;
  {
    u8 included[MAX_BOXES] = {0};

    for (i32 i = 0; i < distance_count; i++) {
      Pair cur = pairs[i];
      included[cur.idx[0]] = 1;
      included[cur.idx[1]] = 1;

      i32 all_included = 1;
      for (i32 j = 0; j < box_count; j++) {
        if (!included[j]) {
          all_included = 0;
          break;
        }
      }

      if (all_included) {
        answer2 = boxes[cur.idx[0]].data[0] * boxes[cur.idx[1]].data[0];
        break;
      }
    }
  }

  // Part one
  i64 answer1 = 0;
  {
    i32 best[3] = { 1, 1, 1, };

    i32 visited = npairs;
    while (visited != 0) {
      i32 len = flood(pairs, 0, &visited);

      // This is some very advanced (read ugly bullshit) code.
      i32 count = 2;
      for (i32 i = 1; i < len; i++) {
        Pair cur = pairs[visited+i];

        {
          i32 needle = cur.idx[0];

          i32 seen = 0;
          for (i32 j = 0; j < i; j++) {
            Pair prev = pairs[visited+j];
            if (prev.idx[0] == needle || prev.idx[1] == needle) {
              seen = 1;
              break;
            }
          }

          if (!seen) {
            count++;
          }
        }

        {
          i32 needle = cur.idx[1];

          i32 seen = 0;
          for (i32 j = 0; j < i; j++) {
            Pair prev = pairs[visited+j];
            if (prev.idx[0] == needle || prev.idx[1] == needle) {
              seen = 1;
              break;
            }
          }

          if (!seen) {
            count++;
          }
        }
      }

      //printf("%d\n", count);

      if (count > best[0]) { best[0] = count; }

      if (best[0] > best[1]) { swap(best[0], best[1]); }
      if (best[1] > best[2]) { swap(best[1], best[2]); }
    }

    answer1 = best[0] * best[1] * best[2];
  }

  printf("%lld\n%lld\n", answer1, answer2);

  return 0;
}
