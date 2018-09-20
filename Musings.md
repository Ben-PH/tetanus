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

### A volatile personality?
We are playing with a VGABuffer. Because we are "just" writing to it, optimisers will tend to think
of it as a redundant piece of code. Not so. `volatile` will sort that out. Documentation [here](https://docs.rs/volatile/0.2.4/volatile/struct.Volatile.html#method.write)

### Traits to the resue!
So our `write_string` is missing the obvious formatting. We need to `impl core::fmt::write for Writer`
for us to take the easy way out.

All we need to do, is manage the `write_str` method in `core::fmt::write`. this method takes a thing
that you are writing to, and a string. If we just call `self.write_string(s)`, we can as the
definition of `write_str()` for `Writer`, then the `write!` macro will just format `s` for us.

### Be static, my dear Writer.
So having a `pub static WRITER` is interesting, because the compiler complains about 
dereffing raw pointers in constants and other shenanigans. I need to learn WTF is happening...

This WRITER static is actually compile-time, I suspect written directly into the binary. This
limits what you can call to, and we are going outside those limits.

We are also trying to load up `WRITER` with a mutable variable. Defining a mutable in the binary,
yeah, I can see why it's complaining...

This is partly a rust-compiler limitation: "Rust's const evaluator is not able to convert raw
pointers to references at compile time". For now at least.

so, in the mean time, **`lazy_static!` to the rescue**
This `macro_use` boi stops this _compile time_ deficiency and kicks the can down the road to
a _run time_ responsibility.

## Things getting interesting? First Mutex!

Pre-read, we are using `spin` crate, which lets us `use spin::Mutex`. That will let us 
wrap up `WRITER` with a `Mutex<Writer>` type. This will reduce the prevelance of `unsafe`.

...so we have a `vga_buffor` and that has a global `WRITER` interface. Because `WRITER` is
_actually_ a `MUTEX` wrapping up a `Writer`, we can safely access its `Writer` with
`vga_buffer::WRITER.lock()` instead of a naked `Writer`.

#### make unsafe stuff safe!
We have just the one `unsafe` because `&mut *(0xb8000 as *mut Buffer)`. I'm not exactly sure
what's going on here. I'm thinking "well WRITER's buffer is a type. It's an unsafe one. it's
a mutable reference, so ownership of **one**. it has a value of a pointer to a mutable Buffer".
I don't know enough about rust to know what's happening here.

What I **do** understand, is this: Although Writer struct contains `unsafe` in it, we have
engineered safety into it. This is done by wrapping `unsafe` containing type into a `Mutex`.
As a system, this has the flaw of not _guaranteeing_ unsafe code is behind a safe interface,
but provides an _effective_ tooling system to make this straight forward.

## Do~~n't~~ panic.

With our global `Writer` interface allowing us to implement `println!` and `print!`, we can
now get down to writing `panic!` at the disco.

Without going into details of how rust macros work, we setup for three possibles. No, one, or
any other number of args. Well, "all it does is print it like println then inf loop, right?"
...not quite. well, yes, it just does `println!`, but it takes the form of `&PanicInfo` type.
This allows it to give the extra information.
