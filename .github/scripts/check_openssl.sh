#!/bin/bash
set -x

if [ "$(uname)" = "Linux" ]; then
  ldd "$1"
  check=$(ldd "$1" | grep ssl)
  if [ -n "$check" ]; then
    echo "Found dynamic Openssl link"
    exit 1
  fi
else
  otool -L "$1"
  check=$(otool -L "$1" | grep openssl)
  if [ -n "$check" ]; then
    echo "Found dynamic Openssl link"
    exit 1
  fi
fi
