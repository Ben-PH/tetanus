### First go

Implementing panic, it returns the "never" type with a `-> !`
it also takes a reference to a PanicInfo variable.

We also need to replace `main()` with `_start() -> !` again, has 
an inifinate loop (for now).p

`_start()` is led with `pub extern "C"`, This is to ensure the C
calling convention is maintained.

Don't forget, that for Mac and Win, there is a difference in compliation,
I'm not too concerned with that right now.
