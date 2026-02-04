#!/usr/bin/env python3

# script that dumps some unicode chars to copy paste

ranges = [
    (0x2580, 0x259F),  # Block Elements
    (0x1FB00, 0x1FBFF), # Symbols for Legacy Computing Supplement
]

chars_per_line = 40  # 40 chars + 40 spaces = 80 chars per line

count = 0
for start, end in ranges:
    for codepoint in range(start, end + 1):
        print(chr(codepoint), end=' ')
        count += 1
        if count >= chars_per_line:
            print()
            count = 0

if count > 0:
    print()
