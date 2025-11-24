use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::time::Instant;

/*
*  BRAINFUCK LOGIC:
*
*  30,000 cell array
*
*  <, > -> moves to left/right cell
*
*  +/- -> increments/decrements the value in that cell
*
*  [ -> if current cell is zero skip to matcing ]
*
*  ] -> if current cell is NOT zero jump back to starting [
*
*  . -> writes current cell to stdout
*
*  , -> reads user input and stores it to current cell
*
*  and that's it!
*/

fn generate_base(path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("{}.s", path))?;
    let base = "\
.intel_syntax noprefix

.bss
arr:
        .space 30000
input:
        .space 1

.section .text
        .global _start

_start:
        lea r12, [arr]
        add r12, 14999
        mov rdx, 1
        mov byte ptr [r12], 0
";

    file.write(base.as_bytes())?;
    Ok(())
}

fn compile_brainfuck(bf_file: &String, path: &String) -> Result<(), std::io::Error> {
    let mut file = File::open(bf_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut output = OpenOptions::new()
        .append(true)
        .open(format!("{}.s", path))
        .unwrap();
    let mut loop_counter: usize;
    let mut stack_top: usize;
    let mut loop_stack: Vec<usize> = vec![];
    let mut closed_loops: Vec<usize> = vec![];

    for char in contents.chars() {
        match char {
            '>' => writeln!(output, "        add r12, 1")?,
            '<' => writeln!(output, "        sub r12, 1")?,
            '+' => writeln!(output, "        add byte ptr [r12], 1")?,
            '-' => writeln!(output, "        sub byte ptr [r12], 1")?,
            '[' => {
                loop_counter = 0;
                loop {
                    if !loop_stack.contains(&loop_counter) && !closed_loops.contains(&loop_counter)
                    {
                        break;
                    }
                    loop_counter += 1;
                }
                loop_stack.push(loop_counter);
                writeln!(
                    output,
                    "l{}:\n        movzx ecx, byte ptr [r12]\n        cmp ecx, 0\n        je le{}",
                    loop_counter, loop_counter
                )?
            }
            ']' => {
                let stack_length = loop_stack.len();
                if stack_length > 0 {
                    stack_top = loop_stack[stack_length - 1];
                    closed_loops.push(stack_top);
                    loop_stack.pop();
                } else {
                    eprintln!("Syntax error! You cannot close a loop which has not been opened.");
                    if let Err(e) = std::fs::remove_file(format!("{}.s", path)) {
                        eprintln!("Unexpected error occurred: {}", e);
                    }
                    std::process::exit(1);
                }
                writeln!(
                    output,
                    "le{}:\n        movzx ecx, byte ptr [r12]\n        cmp ecx, 0\n        jne l{}",
                    stack_top, stack_top
                )?
            }
            '.' => writeln!(
                output,
                "        mov rax, 1\n        mov rdi, 1\n        lea rsi, [r12]\n        syscall"
            )?,
            ',' => writeln!(
                output,
                "        mov rax, 0\n        mov rdi, 0\n        lea rsi, [input]\n        syscall\n        mov cl, [input]\n        mov [r12], cl"
            )?,
            _ => continue,
        };
    }

    if loop_stack.len() > 0 {
        eprintln!("Syntax error! You have declared a loop which is never closed.");
        if let Err(e) = std::fs::remove_file(format!("{}.s", path)) {
            eprintln!("Unexpected error occurred: {}", e);
        }
        std::process::exit(1);
    }
    writeln!(
        output,
        "        mov rax, 60\n        mov rdi, 0\n        syscall"
    )?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Error: Invalid argument(s).\nUSAGE: bfc [BRAINFUCK FILE].bf [OUTPUT FILE]");
        std::process::exit(1);
    }

    let brainfuck_file = &args[1];
    let output_file = &args[2];
    let now = Instant::now();

    if let Err(e) = generate_base(&output_file) {
        eprintln!("Error creating output file: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = compile_brainfuck(&brainfuck_file, &output_file) {
        eprintln!("Error compiling brainfuck: {}", e);
        std::process::exit(1);
    }

    println!(
        "Program compiled in {} millisecond(s).",
        now.elapsed().as_millis()
    );

    let cmd = Command::new("gcc")
        .arg(format!("{}.s", output_file))
        .arg("-o")
        .arg(output_file)
        .arg("-nostdlib")
        .arg("-static")
        .output()
        .expect("Assembling/linking failed! Do you have GCC installed?");

    if !cmd.status.success() {
        eprintln!("GCC failed to assemble/link:");
        eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));
        std::process::exit(1);
    }

    if let Err(e) = std::fs::remove_file(format!("{}.s", output_file)) {
        eprintln!("Unexpected error occurred: {}", e);
        std::process::exit(1);
    }
}
