<p align="center">
    <img src="https://d1drfx3idpovxr.cloudfront.net/squid-vm.svg" alt="SquidVM Logo" width="350px" />
</p>

![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/t/Fragmenta-Company/SquidVM/production)
![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/Fragmenta-Company/SquidVM)
![GitHub (Pre-)Release Date](https://img.shields.io/github/release-date-pre/Fragmenta-Company/SquidVM?label=last%20pre-release)
![GitHub License](https://img.shields.io/github/license/Fragmenta-Company/SquidVM)

# **What is SquidVM?**

SquidVM is a Stack-based VM (just like _Java's JVM_).
It's made from the ground up using Rust for its implementation.

The VM will contain a programming language with the name Squid.

Follow these to know how to use it:
- Type `./squid-vm(.exe) --help` 
- [Access documentation here](https://squidvmdocs.fragmenta.org/)

## What is a .sqdbin file?

It's the binary file that contains the instructions and data the VM will 
use to run your program!

Want to know more about the instruction set, stack and heap implemenation?
[Access the documentation here](https://squidvmdocs.fragmenta.org/).

I'll probably make a simple wiki for that in some time! _Just wait for it._

## How to open .sqdbin files

### As of [SquidVM 0.6.0-alpha](https://github.com/Fragmenta-Company/SquidVM/releases/tag/V0.6.0-alpha):

The command require tags now!

For binary files:
`./squid-vm(.exe) -b testbinary`

For SARs:
`./squid-vm(.exe) -s sarfile`

Both doesn't need the extension to be typed.
But it's still there.

### For [versions between 0.4.0 (pre-alpha) and 0.5.2-alpha](https://github.com/Fragmenta-Company/SquidVM/compare/V0.4.0...V0.6.0-alpha):

Run it like this
`./squid-vm testbinary.sqdbin`

Here ya go!

Hope it helps. : )

## What are .sar files?
SARs or Squid ARchives are files that are a group of binary files combined
into a single archive.

### So the files can be compressed or encrypted?
For now it's just a plan to add, but yes, if all goes well, it will have
support for all that and more in the future.

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when SARs are made available!

## What is Spark?

### Spark is the benchmarking and profiling tool for the SquidVM.
#### It can be called like that:

* `./squidvm spark bench ./binary.sqdbin`

* `./squidvm spark profile ./binary.sqdbin`

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when Spark is ready!

`todo!();`