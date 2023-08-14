use rand::{rngs::ThreadRng, Rng};
use std::collections::HashSet;

pub fn connect_stubs_diagonal_rand_descend(degrees: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> (Vec<(usize,usize)>, usize) {
    // order and remove zeros
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    // println!("edge list: {:?}",edge_list);
    let mut nodes: Vec<(usize, usize)> = degrees
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    // println!("nodes: {:?}",nodes);
    nodes.sort_by(|a,b| b.1.cmp(&a.1));
    // println!("nodes: {:?}", nodes);
    // go through node list from largest degree to smallest connecting and removing at each step
    let mut missing_links: usize = 0;
    while nodes.len() > 1 {
        let i = rng.gen_range(1..nodes.len());
        edge_list.push((nodes[0].0,nodes[i].0));
        //reduce or delete target
        match nodes[i].1 {
            0..=1 => {
                nodes.remove(i);
                // println!("node removed... nodes: {:?}", nodes);
            },
            _ => {
                nodes[i].1 -= 1;
                // println!("nodes degree reduced: {:?}", nodes);
            }
        }
        //reduce or delete index node
        match nodes[0].1 {
            0..=1 => {
                nodes.remove(0);
            },
            _ => {
                nodes[0].1 -= 1;
            }
        }
        move_element(&mut nodes);
    }
    missing_links += nodes.iter().map(|(_,x)| *x).sum::<usize>();
    // println!("edge list: {:?}, \n missing links: {:?} \n degrees: {:?}", edge_list, missing_links, degrees);
    (edge_list, missing_links)
}

pub fn connect_stubs_diagonal_rand_ascend(degrees: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> (Vec<(usize,usize)>, usize) {
    // order and remove zeros
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    // println!("edge list: {:?}",edge_list);
    let mut nodes: Vec<(usize, usize)> = degrees
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    // println!("nodes: {:?}",nodes);
    nodes.sort_by(|a,b| a.1.cmp(&b.1));
    // println!("nodes: {:?}", nodes);
    // go through node list from largest degree to smallest connecting and removing at each step
    let mut missing_links: usize = 0;
    while nodes.len() > 1 {
        let mut leftovers = nodes[0].1;
        // println!("leftovers: {}", leftovers);
        let mut connections: HashSet<usize> = HashSet::new();
        while connections.len() < leftovers.min(nodes.len() - 1) {
            connections.insert(rng.gen_range(1..nodes.len()));
            leftovers -= 1;
        }
        for &i in &connections {
            edge_list.push((nodes[0].0,nodes[i].0));
        }
        // remove nodes and change degrees
        let mut used_nodes: Vec<usize> = connections.into_iter().collect::<Vec<usize>>();
        used_nodes.sort_unstable_by(|a, b| b.cmp(a));
        for &i in used_nodes.iter() {
            match nodes[i].1 {
                0..=1 => {
                    nodes.remove(i);
                    // println!("node removed... nodes: {:?}", nodes);
                },
                _ => {
                    nodes[i].1 -= 1;
                    // println!("nodes degree reduced: {:?}", nodes);
                }
            }
        }
        missing_links += leftovers;
        // println!("missing links tmp: {:?}", leftovers);
        nodes.remove(0);
        // println!("remove myself... nodes: {:?}", nodes);
        nodes.sort_by(|a,b| a.1.cmp(&b.1));
        // println!("nodes: {:?}", nodes);
    }
    // println!("edge list: {:?}, \n missing links: {:?} \n degrees: {:?}", edge_list, missing_links, degrees);
    (edge_list, missing_links)
}

pub fn connect_stubs_rand_descend(degrees1: &Vec<(usize, usize)>, degrees2: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> (Vec<(usize,usize)>, usize) {
    // order and remove zeros
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    let mut degrees_a: Vec<(usize, usize)> = degrees1
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    degrees_a.sort_by(|a,b| b.1.cmp(&a.1));
    // println!("degrees a: {:?}", degrees_a);
    // order and remove zeros
    let mut degrees_b: Vec<(usize, usize)> = degrees2
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    // println!("degrees b: {:?}", degrees_b);

    // loop through a and b
    let mut missing_links: usize = 0;
    while degrees_a.len() > 0 && degrees_b.len() > 0 {

        let i = rng.gen_range(0..degrees_b.len());
        edge_list.push((degrees_a[0].0,degrees_b[i].0));
        //reduce or delete target
        match degrees_b[i].1 {
            0..=1 => {
                degrees_b.remove(i);
                // println!("node removed... nodes: {:?}", nodes);
            },
            _ => {
                degrees_b[i].1 -= 1;
                // println!("nodes degree reduced: {:?}", nodes);
            }
        }
        //reduce or delete index node
        match degrees_a[0].1 {
            0..=1 => {
                degrees_a.remove(0);
            },
            _ => {
                degrees_a[0].1 -= 1;
            }
        }
        move_element(&mut degrees_a);
    }
    missing_links += degrees_a.iter().map(|(_, x)| *x).sum::<usize>();
    missing_links += degrees_b.iter().map(|(_, x)| *x).sum::<usize>();
    (edge_list, missing_links)
}

pub fn connect_stubs_rand_ascend(degrees1: &Vec<(usize, usize)>, degrees2: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> (Vec<(usize,usize)>, usize) {
    // order and remove zeros
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    let mut degrees_a: Vec<(usize, usize)> = degrees1
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    degrees_a.sort_by(|a,b| a.1.cmp(&b.1));
    // println!("degrees a: {:?}", degrees_a);
    // order and remove zeros
    let mut degrees_b: Vec<(usize, usize)> = degrees2
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    // println!("degrees b: {:?}", degrees_b);

    // loop through a and b
    let mut missing_links: usize = 0;
    while degrees_a.len() > 0 && degrees_b.len() > 0 {
        let mut leftovers = degrees_a[0].1;
        // println!("leftovers: {}", leftovers);
        let mut connections: HashSet<usize> = HashSet::new();
        while connections.len() < leftovers.min(degrees_b.len() - 1) {
            connections.insert(rng.gen_range(1..degrees_b.len()));
            leftovers -= 1;
        }
        for &i in &connections {
            edge_list.push((degrees_a[0].0,degrees_b[i].0));
        }
        // remove nodes and change degrees
        let mut used_nodes: Vec<usize> = connections.into_iter().collect::<Vec<usize>>();
        used_nodes.sort_unstable_by(|a, b| b.cmp(a));
        for &i in used_nodes.iter() {
            match degrees_b[i].1 {
                0..=1 => {
                    degrees_b.remove(i);
                    // println!("node removed... nodes: {:?}", nodes);
                },
                _ => {
                    degrees_b[i].1 -= 1;
                    // println!("nodes degree reduced: {:?}", nodes);
                }
            }
        }
        missing_links += leftovers;
        // println!("missing links tmp: {:?}", leftovers);
        degrees_a.remove(0);
        // println!("remove myself... degrees a: {:?}", degrees_a);
        degrees_a.sort_by(|a,b| a.1.cmp(&b.1));
        // degrees_b.sort_by(|a,b| b.1.cmp(&a.1));
        // println!("degrees a: {:?}", degrees_a);
        // println!("degrees b: {:?}", degrees_b);
    }
    missing_links += degrees_b.iter().map(|(_, x)| *x).sum::<usize>();
    // println!("missed stubs: {}", missing_links);
    // println!("edge list: {:?}", edge_list);
    (edge_list, missing_links)
}

pub fn connect_stubs_diagonal_assort(degrees: &Vec<(usize, usize)>) -> (Vec<(usize,usize)>, usize) {
    // order and remove zeros
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    // println!("edge list: {:?}",edge_list);
    let mut nodes: Vec<(usize, usize)> = degrees
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    // println!("nodes: {:?}",nodes);
    nodes.sort_by(|a,b| b.1.cmp(&a.1));
    // println!("nodes: {:?}", nodes);
    // go through node list from largest degree to smallest connecting and removing at each step
    let mut missing_links: usize = 0;
    while nodes.len() > 1 {
        let mut leftovers = nodes[0].1;
        // println!("leftovers: {}", leftovers);
        let mut used_nodes: Vec<usize> = Vec::new();
        for i in 1..(if nodes[0].1 + 1 > nodes.len() {nodes.len()} else {nodes[0].1 + 1}) {
            edge_list.push((nodes[0].0,nodes[i].0));
            // println!("edge list: {:?}", edge_list);
            leftovers -= 1;
            // println!("leftovers: {}", leftovers);
            used_nodes.push(i);
            // println!("used nodes: {:?}", used_nodes);
        }
        // remove nodes and change degrees
        for &i in used_nodes.iter().rev() {
            match nodes[i].1 {
                0..=1 => {
                    nodes.remove(i);
                    // println!("node removed... nodes: {:?}", nodes);
                },
                _ => {
                    nodes[i].1 -= 1;
                    // println!("nodes degree reduced: {:?}", nodes);
                }
            }
        }
        missing_links += leftovers;
        // println!("missing links tmp: {:?}", leftovers);
        nodes.remove(0);
        // println!("remove myself... nodes: {:?}", nodes);
        nodes.sort_by(|a,b| b.1.cmp(&a.1));
        // println!("nodes: {:?}", nodes);
    }
    // println!("edge list: {:?}, \n missing links: {:?} \n degrees: {:?}", edge_list, missing_links, degrees);
    (edge_list, missing_links)
}

pub fn connect_stubs_assort(degrees1: &Vec<(usize, usize)>, degrees2: &Vec<(usize, usize)>) -> (Vec<(usize,usize)>, usize) {
    // order and remove zeros
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    let mut degrees_a: Vec<(usize, usize)> = degrees1
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    degrees_a.sort_by(|a,b| b.1.cmp(&a.1));
    // println!("degrees a: {:?}", degrees_a);
    // order and remove zeros
    let mut degrees_b: Vec<(usize, usize)> = degrees2
        .iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i,x)| (*i, *x))
        .collect();
    degrees_b.sort_by(|a,b| b.1.cmp(&a.1));
    // println!("degrees b: {:?}", degrees_b);
    // loop through a and b
    let mut missing_links: usize = 0;
    while degrees_a.len() > 0 && degrees_b.len() > 0 {
        let mut leftovers = degrees_a[0].1;
        // println!("leftovers: {}", leftovers);
        let mut used_nodes: Vec<usize> = Vec::new();
        for i in 0..(if degrees_a[0].1 > degrees_b.len() {degrees_b.len()} else {degrees_a[0].1}) {
            edge_list.push((degrees_a[0].0, degrees_b[i].0));
            // println!("edge list: {:?}", edge_list);
            leftovers -= 1;
            // println!("leftovers: {}", leftovers);
            used_nodes.push(i);
            // println!("used nodes: {:?}", used_nodes);
        }
        // remove nodes and change degrees
        for &i in used_nodes.iter().rev() {
            match degrees_b[i].1 {
                0..=1 => {
                    degrees_b.remove(i);
                    // println!("degrees b removed... nodes: {:?}", degrees_b);
                },
                _ => {
                    degrees_b[i].1 -= 1;
                    // println!("degrees b degree reduced: {:?}", degrees_b);
                }
            }
        }
        missing_links += leftovers;
        // println!("missing links tmp: {:?}", leftovers);
        degrees_a.remove(0);
        // println!("remove myself... degrees a: {:?}", degrees_a);
        degrees_a.sort_by(|a,b| b.1.cmp(&a.1));
        degrees_b.sort_by(|a,b| b.1.cmp(&a.1));
        // println!("degrees a: {:?}", degrees_a);
        // println!("degrees b: {:?}", degrees_b);
    }
    missing_links += degrees_b.iter().map(|(_, x)| *x).sum::<usize>();
    // println!("missed stubs: {}", missing_links);
    // println!("edge list: {:?}", edge_list);
    (edge_list, missing_links)
}

fn move_element(vec: &mut Vec<(usize,usize)>) {
    if vec.is_empty() {
        return 
    }

    let first = vec[0].1;
    let mut i = 1;
    while i < vec.len() && vec[i].1 > first {
        vec.swap(i - 1, i);
        i += 1;
    }
}