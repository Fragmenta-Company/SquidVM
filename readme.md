<p align="center">
    <img src="https://d1drfx3idpovxr.cloudfront.net/squid-vm.svg" alt="SquidVM Logo" width="350px" />
</p>

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

![GitHub commit activity (branch)](https://img.shields.io/github/commit-activity/t/Fragmenta-Company/SquidVM/main)
![GitHub last commit (by committer)](https://img.shields.io/github/last-commit/Fragmenta-Company/SquidVM)
![GitHub Release Date](https://img.shields.io/github/release-date/Fragmenta-Company/SquidVM?label=last%20release)
![GitHub License](https://img.shields.io/github/license/Fragmenta-Company/SquidVM)

# **What is SquidVM?**

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

SquidVM is a Stack-based VM (just like _Java's JVM_).
It's made from the ground up using Rust for its implementation.

The VM will contain a programming language with the name Squid.

## How to install it?

### Linux

Download the installation script for:
- [SquidVM Runtime (SVRT)](https://github.com/Fragmenta-Company/SquidVM/raw/production/install-scripts/install-squidvm.sh)
- [SquidVM Development Kit (SVDK)](https://github.com/Fragmenta-Company/SquidVM/raw/production/install-scripts/install-svdk.sh)

After that, you will need to make the script executable:

```shell
foo@bar:~$ chmod +x ./install-svdk.sh
```

Then run it normally:
```shell
foo@bar:~$ sudo ./install-svdk.sh
```

### Done!

### Windows

Download the setup from the latest releases for:
- [SquidVM Runtime (SVRT)](https://github.com/Fragmenta-Company/SquidVM/releases/latest/download/SVDK.Setup.exe)
- [SquidVM Development Kit (SVDK)](https://github.com/Fragmenta-Company/SquidVM/releases/latest/download/SquidVM.Setup.exe)

Open the file after the download and install it.
### Done!
The installer tries to add SquidVM to PATH, so try using SquidVM
in the terminal, and tell me if it works on your machine!

### MacOS

```rust
todo!();
```

## How to use?

If you are trying to run your compiled Squid code:

Type this into the terminal:
```shell
foo@bar:~$ squidvm --help
```
If you are a VM developer or trying to debug a bit:

Type this instead:
```shell
foo@bar:~$ svdk --help
```
- [Access documentation here](https://squidvmdocs.fragmenta.org/)
to get more info about the internal function of the VM.

## What is a .sqdbin file?

It's the binary file that contains the instructions and data the VM will 
use to run your program!

Want to know more about the instruction set, stack and heap implemenation?
[Access the documentation here](https://squidvmdocs.fragmenta.org/).

I'll probably make a simple wiki for that in some time! _Just wait for it._

## How to open .sqdbin files

### As of [SquidVM 0.6.0-alpha](https://github.com/Fragmenta-Company/SquidVM/releases/tag/V0.6.0-alpha):

The command requires tags now!

For binary files:
```shell
foo@bar:~$ squid-vm -b testbinary
```
For SARs:
```shell
foo@bar:~$ squid-vm -s sarfile
```

Both don't need the extension to be typed.
But it's still there.

### For [versions between 0.4.0 (pre-alpha) and 0.5.2-alpha](https://github.com/Fragmenta-Company/SquidVM/compare/V0.4.0...V0.6.0-alpha):

Run it like this
```shell
foo@bar:~$ ./squid-vm(.exe) testbinary.sqdbin
```

Here ya go!

Hope it helps. : )

## What are .sar files?
SARs or Squid ARchives are files that are a group of binary files combined
into a single archive.

### So the files can be compressed or encrypted?
For now, it's just a plan to add, but yes, if all goes well, it will have
support for all that and more in the future.

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when SARs are made available!

## What is Spark?

### Spark is the benchmarking and profiling tool for the SquidVM.
#### It can be called like that:

```shell
foo@bar:~$ squidvm --spark bench -b ./binary.sqdbin
```
and
```shell
foo@bar:~$ squidvm --spark profile -b ./binary.sqdbin
```

**IT'S NOT IMPLEMENTED YET!**

The above warning will be removed when Spark is ready!

```rust
todo!();
```