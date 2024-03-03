mod node;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use std::io;
use crate::node::Node;
use crate::node::InsistLevel;
use rayon::prelude::*;
use std::collections::HashSet;

fn main() {
    let mut squares: Vec<Node> = Vec::new();
    let size: u32;
    let found = Arc::new(AtomicBool::new(false));
    let start = Instant::now();

    const NUM_THREADS: u16 = 4;
    rayon::ThreadPoolBuilder::new().num_threads(NUM_THREADS as usize).build_global().unwrap();
    
    println!("Enter the size of the square:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    size = input.trim().parse().expect("Invalid input");
    fill_squares(&mut squares, size as u16,NUM_THREADS);

    while !found.load(Ordering::Acquire) 
    {
        squares.par_iter_mut().for_each(|sq|
        {
            let chance_percent = sq.get_chance_percent();
            // println!("chance: {} ,    percent: {} ,     len: {}", sq.total_chance.to_string(), sq.get_chance_percent(), sq.total_chance_history.len().to_string());
            // sleep(Duration::from_millis(500));

            if sq.is_result() {
                found.store(true, Ordering::Release);
                show_square(sq);
                return;
            }
           
            
            let should_insist: InsistLevel = {
                
                if sq.total_chance_history >  ( size.pow(4) * (chance_percent as u32 + size) * 10 ) 
                {
                    if sq.total_chance_history % 2 == 0 {
                        InsistLevel::DontCheck
                    } 
                    else {
                        InsistLevel::Any
                    }
                } 
                else {
                    InsistLevel::Equal
                }
            };
            sq.swap_two_random_elements(should_insist);
        });
    }


    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed.as_secs());
    //pause execution
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn fill_squares(squares: &mut Vec<Node>, size: u16, node_count: u16) {
    let mut rng = rand::thread_rng();
    for _i in 0..node_count {
        let mut node: Node = Node::new(size);
        let mut buffer: HashSet<u16> = HashSet::new();
        for k in 0..size {
            for j in 0..size {
                loop {
                    let rand_num: u16 = rng.gen_range(1..= size * size);
                    if !buffer.contains(&rand_num) {
                        node.square[[k as usize,j as usize]] = rand_num;
                        buffer.insert(rand_num);
                        break;
                    }
                }
            }
        }
        node.guess_chance(true);
        squares.push(node);
    }
}

fn show_square(node: &Node) {
    for k in 0..node.size {
        for j in 0..node.size {
            print!("{:<4}", node.square[[k as usize,j as usize]]);
        }
        println!();
    }
    println!();
}
