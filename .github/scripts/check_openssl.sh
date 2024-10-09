#!/bin/bash
set -ex

if [ "$(uname)" = "Linux" ]; then
  ldd "$1" | grep ssl && exit 1
else
  otool -L "$1" | grep openssl && exit 1
fi
