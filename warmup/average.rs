use std::{os, float};

fn main() {
    let args: ~[~str] = os::args();
    let mut sum: float = 0.0;
    let mut count = args.len() - 1;
    for std::uint::range(1, args.len()) |i| {
        match float::from_str(args[i]) {
            Some(num) => {sum += num;}
            None => {
		count -= 1;
                println(fmt!("Bad input: %s", args[i]));
	    }
	}
    }
    println(fmt!("Average: %f", sum / (count as float)));
}
