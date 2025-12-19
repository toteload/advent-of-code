@echo off

if "%1"=="" (
  echo Usage "build <day-number>"
  goto :end
)

set padded=0%1
set num=%padded:~-2%
clang-cl -W4 -Zi -Oi -O2 day%num%.c -o day%num%.exe

:end
