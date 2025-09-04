#!/usr/bin/env bash
set -e

if [ "$1" = "hello" ]; then
  echo "cursorctl: hello"
else
  echo "cursorctl: unknown command"
  exit 1
fi
