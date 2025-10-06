#!/usr/bin/env cracked
; Example pipeline script

; List all .txt files in current directory, sorted by size
(pipe
  (find "." "-name" "*.txt" "-type" "f")
  (xargs "ls" "-lh")
  (sort "-k5" "-h"))
