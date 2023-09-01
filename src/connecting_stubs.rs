use rand::{rngs::ThreadRng, Rng};
use std::collections::HashSet;

pub fn cleanup_single(source: &Vec<(usize, usize)>, target: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> 
    (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        // order and remove zeros
        let mut edge_list: Vec<(usize, usize)> = Vec::new();
        let mut source: Vec<(usize, usize)> = source
            .iter()
            .filter(|(_, x)| *x != 0)
            .map(|(i,x)| (*i, *x))
            .collect();
        source.sort_by(|a,b| b.1.cmp(&a.1));

        let mut target: Vec<(usize, usize)> = target
            .iter()
            .filter(|(_, x)| *x != 0)
            .map(|(i,x)| (*i, *x))
            .collect();


        while source.len() > 0 && target.len() > 0  {
            edge_list.push(connect_stub(&mut source, &mut target, rng));
        } 

    (edge_list, source, target)
    }


pub fn cleanup_double(source: &Vec<(usize, usize)>, target1: &Vec<(usize, usize)>, target2: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> 
    (Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        // order and remove zeros
        let mut edge_list: Vec<(usize, usize)> = Vec::new();
        let mut source: Vec<(usize, usize)> = source
            .iter()
            .filter(|(_, x)| *x != 0)
            .map(|(i,x)| (*i, *x))
            .collect();
        source.sort_by(|a,b| b.1.cmp(&a.1));

        let mut target1: Vec<(usize, usize)> = target1
            .iter()
            .filter(|(_, x)| *x != 0)
            .map(|(i,x)| (*i, *x))
            .collect();
        // target1.sort_by(|a,b| b.1.cmp(&a.1));

        let mut target2: Vec<(usize, usize)> = target2
            .iter()
            .filter(|(_, x)| *x != 0)
            .map(|(i,x)| (*i, *x))
            .collect();
        // target2.sort_by(|a,b| b.1.cmp(&a.1));

        while source.len() > 0 && (target1.len() > 0 || target2.len() > 0) {
            if target1.len() > 0 && target2.len() > 0 {
                let neighbour: usize = rng.gen_range(0..=1);
                match neighbour {
                    0 => {
                        edge_list.push(connect_stub(&mut source, &mut target1, rng));
                    },
                    1 => {
                        edge_list.push(connect_stub(&mut source, &mut target2, rng));
                    },
                    _ => {println!("oh no")}
                }
            }
            else {
                match target1.len() {
                    0 => {
                        edge_list.push(connect_stub(&mut source, &mut target2, rng));
                    },
                    _ => {
                        edge_list.push(connect_stub(&mut source, &mut target1, rng));
                    }
                }
            }
        } 

    (edge_list, source, target1, target2)
    }

pub fn connect_stubs_diagonal_rand_descend(degrees: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> 
    (Vec<(usize,usize)>, (usize,usize), (Vec<(usize,usize)>, Vec<(usize,usize)>)) {
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
    (edge_list, (missing_links, missing_links), (nodes.clone(),nodes))
}

pub fn connect_stubs_diagonal_rand_ascend(degrees: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> (Vec<(usize,usize)>, (usize,usize)) {
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
    (edge_list, (missing_links, missing_links))
}

pub fn connect_stubs_rand_descend(degrees1: &Vec<(usize, usize)>, degrees2: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> 
    (Vec<(usize,usize)>, (usize,usize), (Vec<(usize,usize)>, Vec<(usize,usize)>)) {

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

    (edge_list, (degrees_a.iter().map(|(_, x)| *x).sum::<usize>(), degrees_b.iter().map(|(_, x)| *x).sum::<usize>()), (degrees_a, degrees_b))
}

pub fn connect_stubs_rand_ascend(degrees1: &Vec<(usize, usize)>, degrees2: &Vec<(usize, usize)>, rng: &mut ThreadRng) -> (Vec<(usize,usize)>, (usize,usize)) {
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
    // missing_links += degrees_b.iter().map(|(_, x)| *x).sum::<usize>();
    // println!("missed stubs: {}", missing_links);
    // println!("edge list: {:?}", edge_list);
    (edge_list, (missing_links, degrees_b.iter().map(|(_, x)| *x).sum::<usize>()))
}

pub fn connect_stubs_diagonal_assort(degrees: &Vec<(usize, usize)>) -> (Vec<(usize,usize)>, (usize,usize)) {
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
        edge_list.push((nodes[0].0,nodes[1].0));
        //reduce or delete target
        match nodes[1].1 {
            0..=1 => {
                nodes.remove(1);
                // println!("node removed... nodes: {:?}", nodes);
            },
            _ => {
                nodes[1].1 -= 1;
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
        move_second_element(&mut nodes);
        move_element(&mut nodes);
    }
    missing_links += nodes.iter().map(|(_,x)| *x).sum::<usize>();
    // println!("edge list: {:?}, \n missing links: {:?} \n degrees: {:?}", edge_list, missing_links, degrees);
    (edge_list, (missing_links, missing_links))
}

pub fn connect_stubs_assort(degrees1: &Vec<(usize, usize)>, degrees2: &Vec<(usize, usize)>) -> (Vec<(usize,usize)>, (usize,usize)) {
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
        edge_list.push((degrees_a[0].0,degrees_b[0].0));
        //reduce or delete target
        match degrees_b[0].1 {
            0..=1 => {
                degrees_b.remove(0);
                // println!("node removed... nodes: {:?}", nodes);
            },
            _ => {
                degrees_b[0].1 -= 1;
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
        move_element(&mut degrees_b);
    }
    missing_links += degrees_a.iter().map(|(_, x)| *x).sum::<usize>();
    missing_links += degrees_b.iter().map(|(_, x)| *x).sum::<usize>();
    (edge_list, (missing_links, missing_links))
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

fn move_second_element(vec: &mut Vec<(usize, usize)>) {
    if vec.len() < 2 {
        return 
    }

    let second = vec[1].1;
    let mut i = 2;
    while i < vec.len() && vec[i].1 > second {
        vec.swap(i - 1, i);
        i += 1;
    }
}

fn connect_stub(source: &mut Vec<(usize, usize)>, target: &mut Vec<(usize, usize)>, rng: &mut ThreadRng) -> (usize,usize) {
    let i = rng.gen_range(0..target.len());
    let link = (source[0].0,target[i].0);
    //reduce or delete target
    match target[i].1 {
        0..=1 => {
            target.remove(i);
            // println!("node removed... nodes: {:?}", nodes);
        },
        _ => {
            target[i].1 -= 1;
            // println!("nodes degree reduced: {:?}", nodes);
        }
    }
    //reduce or delete index node
    match source[0].1 {
        0..=1 => {
            source.remove(0);
        },
        _ => {
            source[0].1 -= 1;
        }
    }
    move_element(source);

    link
}