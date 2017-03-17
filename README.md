MIX Emulator
============

This emulator is for a *binary MIX machine*. Each 'byte' is 6 bits in size.
The A and X registers, and the main memory, are all 31 bits wide (5 bytes plus a sign bit).
The sign bit is 1 for negative, 0 for positive.
These are stored in the emulator as `u32`.
The I1, I2, ..., I6 and J registers are 13 bits wide, and are stored as `u16`.

# Notes

* Every action you can perform on the `MixMachine` returns a `Result` type.
* For another Mix simulator, see http://danielbarter.github.io/mix.html

# ToDo

* Add tests for store operations.
* Add tests for arithmetic operations.
* Add address transfer operations.
* Add comparison operations.
* Add jump operations.
* Refactor code s.t. the register branches are less biolerplate.
* Refactor the 'helper functions' to convert from 5 byte to 2 byte registers etc. into separate helper classes.
* Add print functionality for the registers? As a helper function?
