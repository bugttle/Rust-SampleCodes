# $ python embed.py

from ctypes import cdll

lib = cdll.LoadLibrary("../target/release/libembed.dylib") #=> for Mac
#lib = cdll.LoadLibrary("../target/release/libembed.so") #=> for Linux

lib.process()

print("done!")
