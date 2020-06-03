#!/usr/bin/env python3
import argparse


def calculate_checksum(file1):
    BIT_LEN = 32
    NUM_INTS = 1 << BIT_LEN         # 0b1000000
    BIT_MASK = NUM_INTS - 1         #  0b111111
    checksum=0;
    for i in range(0, 7):
        n = file1.read(4)
        checksum += int.from_bytes(n, 'little')
    return (checksum ^ BIT_MASK) + 1

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Injects LPC vector checksum in file.')
    parser.add_argument('filename', type=str,
                       help='filename to calculate checksum')
    args = parser.parse_args()
    with open(args.filename, "rb+") as f1:
        checksum = calculate_checksum(f1)
        print(f'Calculated checksum {hex(checksum)}. Writing file...')
        f1.seek(0x1c);
        f1.write(checksum.to_bytes(4, byteorder='little'))
