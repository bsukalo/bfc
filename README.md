# bfc
A naive implementation of a brainfuck to x86-64 GNU assembly compiler, written in Rust.
## Installation
> [!IMPORTANT]
> Please note that this is only compatible with Linux systems using an AMD64 processor.<br />
> **GCC is also required to assemble and link the compiled output!**

Install GCC if you haven't already:

For Debian-based distros:

```
sudo apt install gcc
```

For Arch-based distros:

```
sudo pacman -S gcc
```

Once you have done that:
1. Download the precompiled binary in the [Releases page](https://github.com/bsukalo/bfc/releases/)
2. Mark it as executable:
```
chmod +x bfc
```
3. (Optionally) move it to your PATH:
```
sudo mv bfc /usr/local/bin/
```

## Usage
```
bfc input.bf output.s
gcc output.s -o output -nostdlib -static
./output
```
or in one line:
```bash
bfc input.bf output.s && gcc output.s -o output -nostdlib -static && ./output
```

## Examples & credit
I have included a few fun brainfuck programs in the
[*examples* directory of this repo.](https://github.com/bsukalo/bfc/tree/master/examples) Some of which are:
* **ascii.bf** – Prints all 256 standard + extended ASCII characters
* **btc.bf** by Katie Ball – A brainfuck to C interpreter
* **cat.bf** – A simple cat program, copies stdin to stdout (i.e. writes out whatever you input)
* **hanoi.bf** by Clifford Wolf – A towers of Hanoi problem simulation
* **helloworld.bf** by Katie Ball – You can guess what this one does lol
* **mandelbrot.bf** by Erik Bosman – A mandelbrot set fractal viewer

If you're interested in learning more about brainfuck check out this well-written
[tutorial](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a) by Katie Ball<br />
(the person who made the brainfuck to C interpreter and helloworld program!)
