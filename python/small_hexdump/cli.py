import argparse
import os
import sys
import signal
signal.signal(signal.SIGPIPE, signal.SIG_DFL)

from ._core import hexdump_plain, hexdump_canonical

def main() -> None:
    parser = argparse.ArgumentParser(description="Simple hexdump")
    parser.add_argument("file", help="file to dump")
    parser.add_argument("-k", type=int, default=16, help="bytes per line")
    parser.add_argument("-C", action="store_true", help="canonical output")
    args = parser.parse_args()

    if args.k < 1:
        parser.error("-k must be at least 1")
    
    try:
        with open(args.file, "rb") as f:
            data = f.read()
    except OSError as e:
        sys.exit(f"error: {e}")

    fn = hexdump_canonical if args.C else hexdump_plain
    try:
        print(fn(data, args.k), end="")
        sys.stdout.flush()
    except BrokenPipeError:
        devnull = os.open(os.devnull, os.O_WRONLY)
        os.dup2(devnull, sys.stdout.fileno())
        sys.exit(0)


if __name__ == "__main__":
    main()