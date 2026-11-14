import argparse
import os
import sys

from ._core import *

def main() -> None:
    parser = argparse.ArgumentParser(description="Simple hexdump")
    parser.add_argument("file", help="file to dump")
    parser.add_argument("-k", type=int, default=None, help="bytes per line")
    parser.add_argument("-C", action="store_true", help="canonical output")
    parser.add_argument("-b", action="store_true", help="octal output")
    args = parser.parse_args()

    if args.k is not None and args.k < 1:
        parser.error("-k must be at least 1")
    
    try:
        with open(args.file, "rb") as f:
            data = f.read()
    except OSError as e:
        sys.exit(f"error: {e}")
    
    if(args.C):
        fn = hexdump_canonical
    elif(args.b):
        fn = hexdump_octal 
    else:
        fn = hexdump_plain

    k = args.k if args.k is not None else (16 if args.C or args.b else 8)

    try:
        print(fn(data, k), end="")
        sys.stdout.flush()
    except BrokenPipeError:
        devnull = os.open(os.devnull, os.O_WRONLY)
        os.dup2(devnull, sys.stdout.fileno())
        sys.exit(0)


if __name__ == "__main__":
    main()
