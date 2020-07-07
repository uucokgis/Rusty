/*
Constants and immutable static variables might seem similar, but a subtle difference is that values
in a static variable have a fixed address in memory. Using the value will always access the same data.

Constants, on the other hand, are allowed to duplicate their data whenever they’re used.

Another difference between constants and static variables is that static variables can be mutable.
Accessing and modifying mutable static variables is unsafe.
*/