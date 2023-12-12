<p align="center">
    <img src="https://d1drfx3idpovxr.cloudfront.net/squid-vm.svg" alt="SquidVM Logo" width="350px" />
</p>

![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/t/Fragmenta-Company/SquidVM/production)
![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/Fragmenta-Company/SquidVM)

![GitHub (Pre-)Release Date](https://img.shields.io/github/release-date-pre/Fragmenta-Company/SquidVM?label=Last%20pre-release)
![GitHub License](https://img.shields.io/github/license/Fragmenta-Company/SquidVM)

# **What is SquidVM?**

SquidVM is a Stack-based VM (just like _Java's JVM_).
It's made from the ground up using Rust for its implementation.

The VM will contain a programming language with the name Squid.

## What is a .sqdbin file?

It's the binary file that contains the instructions and data the VM will 
use to run your program!

Want to know more about the instruction set, stack and heap implemenation?

I'll probably make a simple wiki for that in some time! _Just wait for it._

## How to open .sqdbin files

* Execute the VM as normal, via the CLI `./squid-vm` (after using chmod +x of course).
* Put the binary path after the executable.

For example:
`./squid-vm testbinary.sqdbin`

Here ya go!

Hope it helps. : )

## What is Spark?

### Spark is the benchmarking and profiling tool for the SquidVM.
#### It can be called like that:

* `./squidvm spark bench ./binary.sqdbin`

* `./squidvm spark profile ./binary.sqdbin`

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when Spark is ready!

`todo!();`