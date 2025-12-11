#include "util.h"

typedef struct Connection {
  i64 distance;
  i32 idx[2];
  i8 is_visited;
};

Connection *find_insert(Connection *start, Connection *end, i64 d) {
  // TODO
  return NULL;
}

i32 flood(Connection *conns, i32 count, i32 a, i32 b) {
  if (conns[at].is_visited) {
    return 0;
  }

  conns[at].is_visited = true;

  i32 sum = 0;

  for (i32 i = 0; i < count; i++) {
    if (i == at) {
      continue;
    }

    if (conns[i].idx[0] == conns[at].idx[0] || conns[i].idx[1] == conns[at].idx[1]) {

    }
  }

  return 0;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 3) {
    printf("Usage: %s input.txt <pair-count>\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);
  i32 npairs = parse_int(argv[2]);

  i32 boxes[1000 * 3];

  i32 len = 0;
  i32 *box = boxes;
  char const *at = text;
  while (*at) {
    box[0] = parse_int_advance(&at);
    at++; // skip ','
    box[1] = parse_int_advance(&at);
    at++; // skip ','
    box[2] = parse_int_advance(&at);
    at = next_line(at);
    box += 3;
    len++;
  }

  for (i32 i = 0; i < len - 1; i++) {
    for (i32 j = i + 1; j < len; j++) {
      i64 dx = box[i*3] - box[j*3];
      i64 dy = box[i*3+1] - box[j*3+1];
      i64 dz = box[i*3+2] - box[j*3+2];

      i64 d = dx*dx + dy*dy + dz*dz;

      Connection *end = conns + min(i, npairs);
      Connection *insert = find_insert(conns, end, d);
      if (insert == end) {
        continue;
      }

      memmove(/* TODO */);

      *insert = (Connection){
        .distance = d,
        .idx = { i, j, },
        .is_visited = 0,
      };
    }
  }

  for (i32 i = 0; i < npairs; i++) {
    if (conns[i].is_visited) {
      continue;
    }

    i32 size = flood(conns, npairs, i);
  }

  return 0;
}
