### First go

Implementing panic, it returns the "never" type with a `-> !`
it also takes a reference to a PanicInfo variable.

We also need to replace `main()` with `_start() -> !` again, has 
an inifinate loop (for now).p

`_start()` is led with `pub extern "C"`, This is to ensure the C
calling convention is maintained.

Don't forget, that for Mac and Win, there is a difference in compliation,
I'm not too concerned with that right now.


### Min kernel
Looks like we are working on x86. oh boy...

Looks like we will be using `bootimage` to manage the bootloader creation.
I thingk this might be a good idea... We will be using OSFs standard Miltiboot,
we will be referring to GNU GRUB.

We need a Multiboot header


so  `--target` exists, with the comiler using a tripple: `x86_64-unknown-linux-gnu`
architecture, vendor, OS, app. bin. interface (ABI).

We aren't building for the host, though. We are for a _target system_

In terms of the code, we moved the panic information out of `.toml` and into our
.json with `"panic-strategy": "abort"`

the `.json` file has a lot of important things. Without going into detail, it's 
essentially takes the role of the `target-tripple`


## Working VGA.

### All the colors of the rainbow
So first, we needed the `color` enum. Interisting is the `repr(u8)` and `dead_code`
shenanigans. repr, essentially, means that the enum is 8-bit aligned.

### The 80*25 shown in code
We need somewhere to put the data that the scree needs to display. The `char` boing
put onto screen, and the rainbow it expresses. adding `ColorCode` information (`u8`)
to `char` information (`u8`) gives us a `repr(C)` struct that gives `ScreenChar` information
(`repr(C)`, probably 16-bit aligned).

This is enough information to send to a data struct that forms the container for what's up
on the screen. we just need a 80*25 grid that can carry these information structs. This gives
us the `Buffer` struct.


### Doing something useful with the 80*25
So now we have a configured VGA buffer, we need to put information into it.

`Writer` will be our thing for this. We track the index, current ColorCode, and have
a static array to a `VGABuffer`

If we get a `\n`, we `new_line()` dat. If our index reaches our `WIDTH`, do it again, but
we are gonna wait a bit before we `impl` the `new_line()` ~~function~~method

It's clear that writing byte by byte is typical of a helper method. Let's just make `write_str()`
which would make use of it...

we then make `print_something()` that makes use of both functions to make `hello_world()`

As of writing this... [It's working... IT'S WORKING](https://i.ytimg.com/vi/AXwGVXD7qEQ/hqdefault.jpg)

Then, we put in the byte, and ++ the index.

### It ain't a simple compile.

Remember, we aren't doing `cargo build`, we aren't doing just `cargo xbuild --target foo.json`, we
need to be interested in `bootimage build --target foo.json`
