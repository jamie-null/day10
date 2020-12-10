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
    println!("{:?}",adapters);
    let mut chain = Vec::new();
    let mut vstack = Vec::new();
    chain.push(max+3);
    vstack.push(3); //3 step drop from device
    adapters.insert(0); //wall
    let mut jolts = max + 3;
    while jolts > 3 || !adapters.is_empty() {
        /*
        println!("{}",jolts);
        println!("{:?}", chain);
        println!("{:?}",vstack);
        */
        match vstack.pop() {
            Some(4) => { adapters.insert(chain.pop().unwrap()); jolts += vstack.last().unwrap() + 1;}, //expect = panic if chain is empty
            Some(i) => {
                vstack.push(i+1);
                if jolts >= i && adapters.remove(&(jolts - i)) {
                    chain.push(jolts-i);
                    jolts -= i;
                    vstack.push(1);
                }
            }
            None => panic!("Empty vstack"),
        }
    }

    let mut dif: [usize; 3] = [0,0,0];
    let mut last = chain[0];
    for i in chain {
        if i != last {
            dif[last-i-1] += 1;
            last = i;
        }
    }

    println!("{:?}",dif);
    Ok(())
}
