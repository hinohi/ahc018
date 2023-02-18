#!/bin/zsh

set -eu

seq -f '%04g' 0 999 \
  | xargs -P6 -I@ zsh -c './target/release/score @ ./target/release/ahc018 < in/@.txt' \
  | awk '{print $0; s+=$2; l+=log($2)} END{print s, l}'
