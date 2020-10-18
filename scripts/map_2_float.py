#!/usr/local/bin/python3

import sys

def mapFromTo(x, in_min, in_max, out_min, out_max):
   v = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
   return v

if __name__ == "__main__":
   print(mapFromTo(float(sys.argv[1]), float(sys.argv[2]), float(sys.argv[3]), float(sys.argv[4]), float(sys.argv[5])))