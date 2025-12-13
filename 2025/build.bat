@echo off

if "%1"=="" (
  echo Usage "build <day-number>"
  goto :end
)

set padded=0%1
set num=%padded:~-2%
clang -Wall -Wextra -O0 -march=native day%num%.c -o day%num%.exe

:end
