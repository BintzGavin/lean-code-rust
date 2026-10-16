#!/bin/bash

if [ -z "$1" ] || [ "$1" = "--help" ]; then
    echo "Usage: $0 <executable> [args...]"
    exit 0
fi

if ! command -v strace >/dev/null 2>&1; then
    echo "Error: strace is required."
    exit 1
fi

strace -f -e trace=network -o >(grep -E 'AF_INET|AF_INET6') "$@"
