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

## Unit testing. The golden path to awesome.

I have a feeling this will be fun


### configuring for tests
so although a lot of this is "code by numbers", this one particularly so. A lot of the work
here was in managing `#[cfg(...)]` where `...` became `test` and `not(test)`. We also had
to tweak `#![cfg_attr]` to become `#![cfg_attr(not test), no_main]`, It's interesting to me,
because a test run will have two `_start()`'s and one `main()`. We only compile `_start()`
when it's not test, and we only bring in a `main()` when there **is** a test.

### Why we bring in std crate
I'm told the tests run on the host machine (reverting back to having `main()` makes sense),
hence we bring in `std` crate. I'm guessing the host serves the role of qemu when this is
happening.

### the test module
At the bottom of `vga_buffer/mod.rs`, we build our `mod test` code. We have to refer back
to `main.rs`, so we have `use super::*;`. we give make a Writer constructor, which itself
calls a buffer constructor. This is an interesting one, because the naive approach will
fail. the `Volatile` in `Volatile<ScreenChar>` doesn't meet rusts requirements for safe 
array construction. we bring in `array-init` which is a safe interface. As it's in `test`,
we make it a `[dev-dependancies]` in our `.toml`

we can use `array_init(|_| array_init(|_| Volatile::new(empty_char())))`, which I'm _guessing_
safely does the copy. I'm no good with closures (for now...)


Now that we have Writer and buffer constructors, we can do the tests.

The first one, it makes Writer with `construct_writer()`, does a pair of writes using
`writer.write_byte(b'...')` then iterates over the buffer to make sure everything is
as it should be.

The second one uses the `writeln!` macro instead of `write_byte` to write different
strings. It does the same: iteraties over `vga_buffer_chars` with `.iter().enumerated()`
and checks that everything is as it should be.

There is something missing here. Remember how it runs on the host machine? the buffer
sits at `0xb800`. We need to run this in QEMU environment to sort this out.



## Integration testing - "we ain't in software anymore"
We can't see what comes up in QEMU to see how running on "hardware" goes. Rather than
pushing data to the "screen" (memory mapped I/O `0xb8000` for our vga text buffer), we'll
push the data to another memory map - this time a memory mapped port: `port-mapped I/O`.

This uses a separate bus for communication. we take advantage of the cpu's `in` and `out`.
We'll be using UART: `uart_16550` and its crate to abstract away the nitty-gritty.

### Hello vga...
To print from serial, we need to make a static ref, wrap it in a Mutex and... waaaait,
I've seen this before :P

our static ref is a new `SerialPort` number `0x3F8` (1016). We `init()` it, which is relevant
to the need for `lazy_static!` and put it in a Mutex before returning.
We can use this static ref to print: lock it, format the args, and `.expect()` it.

We use that to make a macro (`macro_rules!`) for `serial_print(ln)!`

To run: 
```
> qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin \
    -serial mon:stdio
```
or...
```
bootimage run -- -serial mon:stdio
```

or if you want to output to a file instead of stdout...
```
-serial file:output-file.txt
```

### isa-debug-exit to the resscue

we need to write to one of the ports in the x86_64 architectures IO bus. in this case we
use `0xf4`. We pump 4 bytes into the port with `port.write(0)`. That gets shifted leftt 1, then
the last bit becomes 1: `(bytes << 1) | 1`

Notice how QEMU geos away immediately? probabyl want to hide it altogether, right?

```
bootimage run -- \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none
```



## MOAR EXECUTABLES? This is getting out of hand!

Doing the previous is all well-and-good, until you realize that you don't want to produce it as a product!

`src/bin` is your friend here

in each file in this dir, a new `_start()` is basically a different kernel init. Your `main.rs`
is home to the actual kernel. `src/bin/*` is home to "kernels" (test runs :P ).

We also organized our files a bit better. Notice how `main` is a bit cleaner? 

`lib.rs` holds some good stuff. Basically we've abstracted all the `extern crate` calls into
an `extern crate <self>`, which calls `lib.rs` stuff. In here we have the `extern crate` as
well as the `exit_qemu()` unsafe function. This allows us to just use `extern crate <self>`
in any integration test in an alternate ~~main~~ _start.

If we want to run these tests, `bootimage test` will sort us out!



## What we've done so far

So I moved away from the tutorial for a day or two, and came back. This has taught me a couple of things
 * It emphasised the difference between unit and integration tests in an interesting way.
 * Cemented in my brain how to abstract the core requirements of a system (into `lib.rs`)
  * In doing this, we can build different kernel entries (i.e. `_start()_`)
 * The difference between what's in QEMU output and terminal output (via `serial` library: `serial_print!`)

In terms of what I'm learning about Rust, it's giving me a more practical idea of how to manage a project.
Splitting into files is not just a good idea for ergonomics, it also structures safety. A safe function,
calling unsafe code, polutes its entire scope if use of `unsafe` _becomes_ unsafe.

I'm reading through [this read on unsafe rust](https://doc.rust-lang.org/nomicon) and it's a bit of a revalation...

Anyway. To sum up:
 * To make a binary the kernel way, you need to:
  * `no_std` it
  * `no_main` it then `cargo rustc -- -Z ...` to manage the linker
  * give a `_start()` as the entry point
   * unless you are running it as part of your system and not in QEMU 
  * disable stack unwinding by setting the `\[profile\]` in `.toml` to have `panic = "abort"`
  * implement `panic` with a `-> !` taking `&PanicInfo` arg
  
 * To make an actual Kernel that runs on top of QEMU we must
  * implement a BIOS boot
  * specify a target iwth a `.json`
  * use `cargo xbuild --target ...json` to compile it
  * have `bootlloader_precompiled = "0.2.0"` in our `.toml` dependencies
   * use that in our systems as an `extern crate`
  * now use `bootimage build` where we used to use `cargo xbuild` (same `--target ...`)
  
 * to make a buffer to print to a screen
  * have a `Color` enum, under `repr(u8)`
  * pack two `Color` variables into a `ColorCode` for foreground and background
  * pack a `ColorCode` `u8` into a `ScreenChar` struct
  * make a `Buffer` type to contain a 2d array of `ScreenChar`s
  
 * To start writing to this:
   * a metadata struct `Writer` for the `Buffer`. contains the current `column` and `ColorCode`
    * also carries a static lifetime reference to mutable buffer.
     * "The kernel sees all, knows all, touches all, for all time."
   * `Writer` impl has methods that puts the data into the buffer
 * This is Going to be optimised out by the compiler when we start using it, so...
  * use `extern crate Volatile` and wrap `ScreenChar`s up in the `Buffer` struct
  * we use the `write()` method in the `Volatile` type, taking the `ScreenChar` that we
  want in the argument.
  * Now that we have a way to write to the buffer, we `impl fmt::Write for Writer`
   * Prototyping Writer, we can make a new one that has the buffer ref as `0xb800`
   * it is a mutable reference of a raw ptr, type-casting an adress to `\*mut Buffer`
   * this is `unsafe`
  * A global interface must be inside a `lazy_static!` scope (with `macro_use` and `extern_crate`, etc)
  * it is a type wrapping a `Writer` inside a `Mutex`
  * with this interface available, we can `macro_rules!` the `print(ln)!` macros
  * With these macros, we can now give `panic!` definitions a `println!` usage.
  
 * To set up testing, we:
  * put `#[cfg(not(test))]` above our `panic` and `_start()` impl
  * make sure that we have `main` when testing
  * now we can run `cargo test`
  * we can also silence warnings with a `#![cfg_atr...]`
  * also need to include `extern crate std` when testing
  * we can now define our `mod test {}` code...
  * Don't forget to get access to overything in the test, and to construct your stuff
 
 * An mportant tool for the integration test is the serial port
  * `uart_16550` as a dep
  * make a `mod serial`
  * make a global interface similar to `WRITER`
  * `let mut serial_port = SerialPort::new(0x3f8);` for x86 arch
  * initthe SerialPort object
  * use this object as a new Mutex argument
  * make `serial_print!` macros
  * make `exit_qemu` using `extern crate x85_64`
  * run with `-seria/ mon:stdio -device ... -display none` as needed

 * To set up integration testing:
  * make a `/src/bin` directory to put in your separate executables
  * abstract lines such as `extern crate ...` into `lib.rs`, invoke with `extern crate <self`
  * build a test executable 
  * build with `bootimage run --bin <filename without .rs`
  * annotate your macros with `#[macro_export]` inside your extra library files

##  CPU Excuptions##

### Pre-Reading ###

There are many different things that trigger an exe in the cpu. Some that are straight
forward are div-zero's and page faults. we can bundle them up into a `struct` that forms
an **interuption descriptor table** (given to us by the x86_64 crate).

Like my time with os161, there is a calling convention to be respected. A major clue of
a challange here, is in the name. **interupt**. Doesn't matter what's going on, an interupt
will jump the queu and hog the program counter. OF relevance:
  * 6 registers for the argument - `rdi` `rsi` `rdx` `rcx` `r8` `r9`
  * then the stack
  * results into `rax` and `rdx`
  
**all** preserved registers must be saved - that's because an interupt can occur at any time...
The interupt takes 7 steps
1. Alighn stack pointer (16 bytes)
2. Switch stack
3. Push old SP
4. push and update `RFLAGS` register
5. push IP
6. push err code
7. invoke the handler

we have the `InterruptDescriptorTable` object in the `x86_64` crate to handle most of the details.

The rest is instructional to implement that.


| exception stack frame | size(byte) |
|-----------------------|------------|
| Stack alignment       |          2 |
| Stack Segment         |            |
| SP                    |            |
| RFLAGS                |          4 |
| Code segment          |            |
| Inst P                |            |
| Err Code              |            |
| Stack frame           |            |


### As we go ###

First we `init_idt()`, but we are also referenced to[how debuggers work](https://eli.thegreenplace.net/2011/01/27/how-debuggers-work-part-2-breakpoints "relevant to the `int3` instruction") 

As I added some things, then made the `extern "x86-interrupt" fn breakpoint_handler` I was presented
with an error. Working to fix it, it became clear that there was a deficiency in my understanding
on how to work with cargo, xbuild, bootimage, etc. to build a kernel. It started with `can't find
crate for \`core\`` we came accross this problem all the way back when we first built a free-standing
binary... weird.

I'm not entirely sure what's going on, I'm going to go back in commit history, see where things went different.

