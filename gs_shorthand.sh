#!/bin/sh
## Shorthand for converting pdf to png
## Usage: ./gs_shorthand file.pdf /output/directory/

gs -dNOPAUSE -dBATCH -sDEVICE=png16m -g2000x2828 -dPDFFitPage -dUseCropBox -sOutputFile="$2/page-%d.png" "$1"

