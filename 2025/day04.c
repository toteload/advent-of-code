#include "util.h"
#include <assert.h>
#include <string.h>

void find_dim(char *text, i32 *width, i32 *height, i32 *pitch, i32 *len) {
  char *at = text;
  while (*at && (*at == '.' || *at == '@')) {
    at++;
  }

  *width = (i32)(at - text);

  while (*at && !(*at == '.' || *at == '@')) {
    at++;
  }

  *pitch = (i32)(at - text);
  *len = strlen(text);
  *height = (*len + *pitch - 1) / *pitch;
}

i32 main(i32 argc, char const **argv) {
  if (argc < 2) {
    printf("Usage: %s input.txt\n", argv[0]);
    exit(1);
  }

  char *text = read_file(argv[1]);

  i32 width, height, pitch, len;
  find_dim(text, &width, &height, &pitch, &len);

  {
    i32 count = 0;
    for (i32 cy = 0; cy < height; cy++) {
      for (i32 cx = 0; cx < width; cx++) {
        if (text[cy * pitch + cx] != '@') {
          continue;
        }

        i32 ns[] = {
          -1, -1, 0, -1, 1, -1,
          -1,  0,        1,  0,
          -1,  1, 0,  1, 1,  1,
        };

        i32 neighbor_count = 0;
        for (i32 i = 0; i < 8; i++) {
          i32 x = cx + ns[i * 2];
          i32 y = cy + ns[i * 2 + 1];

          if (x < 0 || x >= width || y < 0 || y >= height) {
            continue;
          }
          
          neighbor_count += text[y * pitch + x] == '@';
        }

        count += neighbor_count < 4;
      }
    }

    printf("%d\n", count);
  }

  {
    char *buf[2] = {
      text,
      malloc(len),
    };

    i32 count = 0;

    i32 curr_idx = 0;
    while (1) {
      i32 next_idx = (curr_idx + 1) % 2;

      char *curr = buf[curr_idx];
      char *next = buf[next_idx];

      curr_idx = next_idx;

      memcpy(next, curr, len);

      i32 removed = 0;

      for (i32 cy = 0; cy < height; cy++) {
        for (i32 cx = 0; cx < width; cx++) {
          if (curr[cy * pitch + cx] != '@') {
            continue;
          }

          i32 ns[] = {
            -1, -1, 0, -1, 1, -1,
            -1,  0,        1,  0,
            -1,  1, 0,  1, 1,  1,
          };

          i32 neighbor_count = 0;
          for (i32 i = 0; i < 8; i++) {
            i32 x = cx + ns[i * 2];
            i32 y = cy + ns[i * 2 + 1];

            if (x < 0 || x >= width || y < 0 || y >= height) {
              continue;
            }
            
            neighbor_count += curr[y * pitch + x] == '@';
          }

	  if (neighbor_count < 4) {
            next[cy * pitch + cx] = '.';
	    removed++;
	  }
        }
      }

      if (!removed) {
        break;
      }

      count += removed;
    }

    printf("%d\n", count);
  }

  return 0;
}
