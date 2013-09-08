use std::{os, uint};

fn main() {
    let args: ~[~str] = os::args();
    let mut n = 1;
    while n < (args.len() - 1) {
	print(args[n]);
	print(" ");
	n += 1;
    }
    print(args[args.len() - 1]);
    println("");
}
