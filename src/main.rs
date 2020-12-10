use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::cmp;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);

    let mut max = 0;
    let mut adapters = HashSet::new();
    for line in buf.lines() {
        let val = line.unwrap().parse::<usize>()?;
        max = cmp::max(max,val);
        if !adapters.insert(val) {
            panic!("Duplicate adapters detected");
        }
    }
    let mut jolts = max + 3;
    let mut dif = [0,0,0];
    let mut chain = Vec::new();
    let mut vstack = Vec::new();
    vstack.push(3);
    while jolts > 3 || !adapters.is_empty() {
        match vstack.pop() {
            Some(0) => {dif[0] -= 1; adapters.insert(chain.pop().unwrap());}, //expect = panic if chain is empty
            Some(i) => {
                vstack.push(i-1);
                if jolts >= i && adapters.remove(&(jolts - i)) {
                    if i < 3 {
                        dif[i] -= 1;
                    }
                    chain.push(jolts-i);
                    jolts -= i;
                    vstack.push(3);
                    dif[i-1] += 1;
                }
            }
            None => panic!("Empty vstack"),
        }
    }
    println!("{}",dif[0] * dif[2]);
    Ok(())
}
