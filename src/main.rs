use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);

    let mut max = 0;
    let mut adapters = HashSet::new();
    for line in buf.lines() {
        let val = line.unwrap().parse::<usize>()?;
        max = cmp::max(max, val);
        if !adapters.insert(val) {
            panic!("Duplicate adapters detected");
        }
    }
    println!("{:?}", adapters);
    let mut chain = Vec::new();
    let mut vstack = Vec::new();
    chain.push(max + 3);
    vstack.push(3); //3 step drop from device
    adapters.insert(0); //wall
    let mut jolts = max + 3;
    //while jolts > 3 || !adapters.is_empty() {
    /*

    */
    let mut combos: HashMap<usize, usize> = HashMap::new();
    combos.insert(0, 1);
    while chain.len() > 1 || *vstack.last().unwrap() != 4 {
        /*
                println!("{}",jolts);
                println!("{:?}", chain);
                println!("{:?}",vstack);

                if jolts <= 3 {
                    combos.insert()
                }
        */
        match vstack.pop() {
            Some(i) if i >= 4 || i > jolts => {
                adapters.insert(chain.pop().unwrap());
                jolts = *chain.last().unwrap();
            } //expect = panic if chain is empty
            Some(i) => {
                vstack.push(i + 1);
                if adapters.remove(&(jolts - i)) {
                    if let Some(c) = combos.get(&(jolts - i)) {
                        let c = c.clone();
                        if let Some(current) = combos.get_mut(&jolts) {
                            *current += c;
                        } else {
                            combos.insert(jolts,c);
                        }
                        adapters.insert(jolts-i);
                    } else {
                        chain.push(jolts - i);
                        jolts -= i;
                        vstack.push(1);
                    }
                }
            }
            None => panic!("Empty vstack"),
        }
    }
    println!("{}", combos.get(&(max)).unwrap());
    /*
    let mut dif: [usize; 3] = [0,0,0];
    let mut last = chain[0];
    for i in chain {
        if i != last {
            dif[last-i-1] += 1;
            last = i;
        }
    }

    println!("{:?}",dif);
    */
    Ok(())
}
