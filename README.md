MIX Emulator
============

This emulator is for a *binary MIX machine*. Each 'byte' is 6 bits in size.
The A and X registers, and the main memory, are all 31 bits wide (5 bytes plus a sign bit).
The sign bit is 1 for negative, 0 for positive.
These are stored in the emulator as `u32`.
The I1, I2, ..., I6 and J registers are 13 bits wide, and are stored as `u16`.

# Notes

* Every action you can perform on the `MixMachine` returns a `Result` type.

# ToDo

* Write tests for load operations.
