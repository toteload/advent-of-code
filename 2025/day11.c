#include "util.h"

#define MAX_DEVICE_COUNT 580
#define MAX_OUTPUT_COUNT 25

typedef struct Device {
  i32 count;
  i32 outputs[MAX_OUTPUT_COUNT];
} Device;

i32 ids[MAX_OUTPUT_COUNT];
Device devices[MAX_DEVICE_COUNT];

i32 read_device(char const *text) {
  i32 d = 0;

  d += text[0] - 'a';
  d *= 26;
  d += text[1] - 'a';
  d *= 26;
  d += text[2] - 'a';

  return d;
}

i32 find_idx(i32 const *ids, i32 count, i32 id) {
  for (i32 i = 0; i < count; i++) {
    if (ids[i] == id) {
      return i;
    }
  }

  return -1;
}

i64 count_paths(i32 from, Device const *devices) {
  // 'out' is put at index zero.
  if (from == 0) {
    return 1;
  }

  i64 sum = 0;

  Device const *dev = devices + from;

  for (i32 i = 0; i < dev->count; i++) {
    sum += count_paths(dev->outputs[i], devices);
  }

  return sum;
}

typedef struct Context {
  i32 idx_fft;
  i32 idx_dac;

  i64 *memo;
} Context;

#define HAS_FFT (1 << 0)
#define HAS_DAC (1 << 1)

i64 count_paths2(Context *ctx, i32 from, u32 flags) {
  // 'out' is put at index zero.
  if (from == 0) {
    return (flags == (HAS_FFT | HAS_DAC));
  }

  i32 idx_memo = 4 * from + flags;
  if (ctx->memo[idx_memo] != -1) {
    return ctx->memo[idx_memo];
  }

  if (from == ctx->idx_fft) {
    flags |= HAS_FFT;
  }

  if (from == ctx->idx_dac) {
    flags |= HAS_DAC;
  }

  i64 sum = 0;

  Device const *dev = devices + from;

  for (i32 i = 0; i < dev->count; i++) {
    sum += count_paths2(ctx, dev->outputs[i], flags);
  }

  ctx->memo[idx_memo] = sum;

  return sum;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  for (i32 i = 0; i < MAX_DEVICE_COUNT; i++) {
    for (i32 j = 0; j < MAX_OUTPUT_COUNT; j++) {
      devices[i].count = 0;
      devices[i].outputs[j] = -1;
    }
  }

  ids[0] = read_device("out");

  char const *at = text;
  i32 count = 1;
  while (*at) {
    ids[count] = read_device(at);
    at += 3;

    at++; // Skip ':'

    i32 output_count = 0;
    while (*at != '\n') {
      at++; // Skip ' '
      
      devices[count].outputs[output_count] = read_device(at);
      at += 3;

      output_count++;
    }

    devices[count].count = output_count;

    count++;

    at = next_line(at);
  }

  for (i32 i = 0; i < count; i++) {
    for (i32 j = 0; j < devices[i].count; j++) {
      devices[i].outputs[j] = find_idx(ids, count, devices[i].outputs[j]);
    }
  }

  // Part one
  {
    i32 idx_you = find_idx(ids, count, read_device("you"));
    i64 answer = count_paths(idx_you, devices);
    printf("%lld\n", answer);
  }

  // Part two
  {
    i32 idx_svr = find_idx(ids, count, read_device("svr"));

    Context ctx = {
      .idx_fft = find_idx(ids, count, read_device("fft")),
      .idx_dac = find_idx(ids, count, read_device("dac")),
      .memo = malloc(sizeof(i64) * 4 * count),
    };

    for (i32 i = 0; i < 4 * count; i++) {
      ctx.memo[i] = -1;
    }

    i64 answer = count_paths2(&ctx, idx_svr, 0);
    printf("%lld\n", answer);
  }

  return 0;
}
