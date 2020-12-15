use std::cmp;
use std::collections::{BTreeMap};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);

    let mut max = 0;
    let mut adapters = Vec::new();
    for line in buf.lines() {
        let val = line.unwrap().parse::<usize>()?;
        max = cmp::max(max, val);
        adapters.push(val);
    }
    adapters.push(max + 3);
    adapters.push(0);
    adapters.sort();

    let mut graph: BTreeMap<usize,Vec<usize>> = BTreeMap::new();
    for jolts in &adapters {
        let next = (1..4).filter(|n| adapters.contains(&(n + jolts)))
                         .map(|n| n + jolts).collect();
        //println!("{:#?}",next);
        graph.insert(*jolts, next);
    }
    let mut combos = BTreeMap::new();
    println!("{}",getcombos(0,&mut graph,&mut combos,max));
    Ok(())
}

fn getcombos(jolts: usize, graph: &mut BTreeMap<usize,Vec<usize>>, combos: &mut BTreeMap<usize,usize>, max: usize) -> usize {
    if jolts == max + 3 {
        1
    } else {
        let mut sum = 0;
        let next = match graph.get(&jolts) {
            Some(a) => a,
            _ => {println!("{}",jolts); unreachable!()},
        };
        for next in next.clone() {
            let next = next;
            if let Some(c) = combos.get(&next) {
                sum += c;
            } else if graph.contains_key(&next) {
                let c = getcombos(next,graph,combos,max);
                sum += c;
                combos.insert(next, c);
            }
        }
        sum
    }
}
