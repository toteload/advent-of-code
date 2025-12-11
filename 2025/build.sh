#!/bin/bash

clang -Wall -Wextra -O2 -march=native "day$(printf "%02d" $1).c" -o "day$(printf "%02d" $1)"
