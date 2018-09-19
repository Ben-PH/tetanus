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
