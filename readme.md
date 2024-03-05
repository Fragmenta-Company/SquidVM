<p align="center">
    <img src="https://d1qrumake2q7xa.cloudfront.net/squid-vm.svg" alt="SquidVM Logo" width="350px" />
</p>

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/t/Fragmenta-Company/SquidVM/main)
![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/Fragmenta-Company/SquidVM)
![GitHub Release Date](https://img.shields.io/github/release-date/Fragmenta-Company/SquidVM?label=last%20release)
![GitHub License](https://img.shields.io/github/license/Fragmenta-Company/SquidVM)

```text
  _____             _     ___      ____  __ 
 / ____|           (_)   | \ \    / /  \/  |
| (___   __ _ _   _ _  __| |\ \  / /| \  / |
 \___ \ / _` | | | | |/ _` | \ \/ / | |\/| |
 ____) | (_| | |_| | | (_| |  \  /  | |  | |
|_____/ \__, |\__,_|_|\__,_|   \/   |_|  |_|
           | |                              
           |_|                              
```

# **What is the SquidVM?**
SquidVM is a Stack-based VM made from the 
ground up using Rust for its implementation.

The VM will contain a programming language with the name _Squid_
and a simpler scripting language called _Squipt_.
Both will be made for working together and also
both will be statically and strongly typed.

The main difference between them is that int,
unsigned int, float, [...] will be replaced with number,
unsafe blocks will not be supported and
some other changes made for simplicity in Squipt.

I'm also thinking of implementing a dynamically typed language.

That one will also be made for working
together with the other two natively.

## How to install it?

### Linux

Download the installation script for:
- [SquidVM Runtime (SVRT)](https://github.com/Fragmenta-Company/SquidVM/raw/production/install-scripts/install-squidvm.sh)
- [SquidVM Development Kit (SVDK)](https://github.com/Fragmenta-Company/SquidVM/raw/production/install-scripts/install-svdk.sh)

After that, you will need to make the script executable:
```shell
foo@bar:~$ chmod +x ./install-svdk.sh
```

Then run the setup:
```shell
foo@bar:~$ sudo ./install-svdk.sh
```

#### For a more specialized setup
I would rather recommend compiling
from source and installing to a desired folder.

### Windows

```rust
todo!();
```

### MacOS

```rust
todo!();
```

## How to use?

If you are trying to run your compiled SquidVM bytecode:

```shell
# Try this command to learn a bit
foo@bar:~$ squidvm --help

# If you are a VM developer or trying to debug
# Try this instead:
foo@bar:~$ svdk --help
```

- [Access documentation here](https://squidvmdocs.fragmenta.org/)
to get more info about the VM's internal function.

## What is a .sqdbin file?

It's the binary file that contains the instructions and data the VM will 
use to run your program!

Want to know more about the instruction set, stack and heap implemenation?
[Access the documentation here](https://squidvmdocs.fragmenta.org/).

I'll probably make a simple wiki for that in some time! _Just wait for it._

## How to open .sqdbin files

### As of [SquidVM 0.6.0-alpha](https://github.com/Fragmenta-Company/SquidVM/releases/tag/V0.6.0-alpha):

The command requires tags now!

```shell
# For binary files
foo@bar:~$ squid-vm -b testbinary
# For SARs
foo@bar:~$ squid-vm -s sarfile
```

Both don't need the extension to be typed.
But it's still there.

### For [versions between 0.4.0 (pre-alpha) and 0.5.2-alpha](https://github.com/Fragmenta-Company/SquidVM/compare/V0.4.0...V0.6.0-alpha):

```shell
# Run like this
foo@bar:~$ ./squid-vm(.exe) testbinary.sqdbin
```

Here ya go!

Hope it helps. : )

## What are .sar files?
SARs or Squid ARchives are files that are a group of binaries combined
into a single archive containing types, metadata and native libraries.

They are made so that instead of sharing lots of
bytecode files each with a different metadata, you
can share a single file containing all the bytecode.

### So SARs can be compressed or encrypted?
For now, it's just a plan, but yes, if all goes right, 
it will have support for all that and more in the future.

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when SARs are made available!

## What is Spark?

### Spark is a benchmarking, profiling and debugging tool for the SquidVM.

#### It will be used like this:
```shell
# For debugging
foo@bar:~$ squidvm spark --debug -b ./binary.sqdbin
# For benchmarking
foo@bar:~$ squidvm spark --bench -b ./binary.sqdbin
# For profiling
foo@bar:~$ squidvm spark --profile -b ./binary.sqdbin
```

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when Spark is ready!

```rust
todo!();
```