use std::collections::{HashMap, VecDeque};

use super::*;

/// To store number of turns
type RankType = DimType;

pub struct Solver1 {
    data: Vec<u8>,
    r_dim: DimType,
    c_dim: DimType,
    start: NodeAdr,
    end: NodeAdr,
}

impl Solver1 {

    pub fn new_rc(r: DimType, c:DimType) -> Result<Self, &'static str> {
        if r < 2 || c < 2 { return Err("Too small dimensions"); }
        // Rough estimation of maximal number of turns 
        // TODO: figure out more exact condition (fun math exercise ;))
        // If this is too taught, set RankType to usize
        if (r as usize + c as usize) / 2 > RankType::MAX as usize { return Err("Too big dimensions"); } 
        Ok(Solver1 { 
            data: vec![0; r as usize * c as usize], 
            r_dim:r, 
            c_dim:c,
            start: (1,0),
            end: (r - 2, c - 1),
        })
    }

    #[inline]
    fn idx(&self, r: DimType, c:DimType) -> usize {
        //if r >= self.r_dim { panic!("Wrong r argument") }
        //if c >= self.c_dim { panic!("Wrong c argument") }
        r as usize * self.c_dim as usize + c as usize
    }

    #[allow(dead_code)]
    #[inline]
    fn rc(&self, idx: usize) -> NodeAdr {
        //if idx >= self.data.len() { panic!("Wrong idx argument") }
        let r = idx / self.c_dim as usize;
        let c = idx - r as usize * self.c_dim as usize;
        (r as DimType, c as DimType)
    }

    #[inline]
    fn is_passage(&self, r: DimType, c:DimType) -> bool {
        self.data[self.idx(r, c)] == 1
    }

    #[inline]
    fn neighbors(&self, rc: NodeAdr) -> NodeIter<'_> {
        NodeIter { solver:&self, r:rc.0, c:rc.1, mode:0 }
    }

}

struct NodeIter<'a> {
    solver: &'a Solver1,
    r: DimType,
    c: DimType,
    mode: u8
}

impl Iterator for NodeIter<'_> {
    type Item = NodeAdr;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.mode {
                0 => {
                    self.mode = 1;
                    if self.r > 0 && self.solver.is_passage(self.r - 1, self.c) {
                        return Some((self.r - 1, self.c));
                    }
                },
                1 => {
                    self.mode = 2;
                    if self.c < self.solver.c_dim - 1 && self.solver.is_passage(self.r, self.c + 1) {
                        return Some((self.r, self.c + 1));
                    }
                },
                2 => {
                    self.mode = 3;
                    if self.r < self.solver.r_dim - 1 && self.solver.is_passage(self.r + 1, self.c) {
                        return Some((self.r + 1, self.c));
                    }
                },
                3 => {
                    self.mode = 4;
                    if self.c > 0 && self.solver.is_passage(self.r, self.c - 1) {
                        return Some((self.r, self.c - 1));
                    }
                },
                _ => { return None; }
            }
        }
    }
}

impl Solver for Solver1 {

    fn set_passage(&mut self, r: DimType, c: DimType) {
        let idx = self.idx(r, c);
        self.data[idx] = 1;
    }

    fn is_valid(&self) -> bool {
        self.data[self.idx(self.start.0, self.start.1)] == 1 
        &&
        self.data[self.idx(self.end.0, self.end.1)] == 1
    }

    fn solve(&self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)> {
        #[derive(Clone)]
        struct Nodedata {
            rank: RankType,
            prev: NodeAdr, 
        }
        
        if !self.is_valid() { return None; }

        // with_capacity() here degrades performance
        let mut purgatory = HashMap::<NodeAdr, Nodedata>::new(); // (r,c) => Nodedata
        let mut heaven = HashMap::<NodeAdr, Nodedata>::new(); // (r,c) => Nodedata
        let mut cur_node = self.start;
        let mut cur_node_data = Nodedata {rank:0, prev:cur_node};
        heaven.insert(cur_node, cur_node_data.clone());
        loop {
            // Evaluate rate of neighbor nodes 
            for node in self.neighbors(cur_node) {
                if let Some(mut node_data) = purgatory.get_mut(&node) {
                    let (r, c) = cur_node_data.prev;
                    if node.0 == cur_node.0 && cur_node.0 == r {
                        // continue moving in vertical direction
                        if node_data.rank > cur_node_data.rank {
                            node_data.rank = cur_node_data.rank;
                            node_data.prev = cur_node;
                        }
                    }
                    else if node.1 == cur_node.1 && cur_node.1 == c {
                        // continue moving in horizontal direction
                        if node_data.rank > cur_node_data.rank {
                            node_data.rank = cur_node_data.rank;
                            node_data.prev = cur_node;
                        }
                    } else {
                        // turn
                        if node_data.rank > cur_node_data.rank + 1 {
                            node_data.rank = cur_node_data.rank + 1;
                            node_data.prev = cur_node;
                        }
                    }
                }
                else if !heaven.contains_key(&node) {
                    let (r, c) = cur_node_data.prev;
                    let init_rank = 
                        if node.0 == cur_node.0 && cur_node.0 == r {
                            // continue moving in vertical direction
                            cur_node_data.rank
                        }
                        else if node.1 == cur_node.1 && cur_node.1 == c {
                            // continue moving in horizontal direction
                            cur_node_data.rank
                        } else {
                            // turn
                            cur_node_data.rank + 1
                        };
                    purgatory.insert(node, Nodedata {rank:init_rank, prev:cur_node});
                }
            }

            // Find todo node with minimal rank
            // TODO: This loop probably could be improved:
            // - maybe use some sorted containers
            // - maybe parallelize
            let mut min_val = RankType::MAX;
            let mut min_node = None;
            for (node, node_data) in purgatory.iter() {
                if node_data.rank < min_val {
                    min_val = node_data.rank;
                    min_node = Some(*node);
                }
            }

            // If list of non fixed nodes is empty or we arrived into end node - exit loop
            if let Some(min_node) = min_node {
                let min_node_data = purgatory.remove(&min_node).expect("Logical error - min_node should be in the purgatory");
                cur_node = min_node;
                cur_node_data = min_node_data.clone();
                heaven.insert(min_node, min_node_data);
                if min_node == self.end {
                    break;
                }
            } else {
                break;
            }
        }

        if let Some(end_node_data) = heaven.get(&self.end) {
            let mut path = VecDeque::new();
            if with_path {
                let mut node = self.end;
                loop {
                    path.push_front(node);
                    let node_data = heaven.get(&node).expect("Logical error - every node on path should be in heaven");
                    if node == self.start { break; }
                    node = node_data.prev;
                    if path.len() > heaven.len() {
                        panic!("Something wrong - Cycle detected");
                        // eprint!("Something wrong - Cycle detected");
                        // break;
                    }
                }
            }
            Some((end_node_data.rank, path))
        } else {
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Tests
/// 

#[cfg(test)]
mod tests {
    use rand::Rng;
    use ndarray::{array, s};
    use ndarray::{Array};
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand_distr::Uniform;
    use super::*;
    use crate::tests::init_from_rand_array;

    #[test]
    fn coordinates_conversion() {
        let solv = Solver1::new_rc(4, 5).unwrap();
        assert_eq!(solv.rc(solv.idx(0, 3)), (0, 3));
        assert_eq!(solv.rc(solv.idx(2, 0)), (2, 0));
        assert_eq!(solv.rc(solv.idx(2, 3)), (2, 3));
        assert_eq!(solv.rc(solv.idx(3, 4)), (3, 4));
    }

    #[test]
    fn passage() {
        let mut solv = Solver1::new_rc(4, 5).unwrap();
        solv.set_passage(2, 3);
        assert_eq!(solv.is_passage(2, 3), true);
        assert_eq!(solv.is_passage(1, 3), false);
    }

    #[test]
    fn neighbors() {
        let arr = array![
            [0,0,1,0,0u8],
            [1,1,1,1,0u8],
            [0,1,1,0,0u8],
            [0,0,0,1,1u8],
        ];
        let adim = arr.shape();
        assert_eq!(adim, &[4, 5]);
        let mut solv = Solver1::new_rc(adim[0] as DimType, adim[1] as DimType).unwrap();
        for (c, v) in arr.indexed_iter() {
            if *v == 1 {
                solv.set_passage(c.0 as DimType, c.1 as DimType);
            }
        }
        assert_eq!(solv.neighbors((1, 1)).collect::<Vec<_>>(), vec![(1, 2), (2, 1), (1, 0)]);
        assert_eq!(solv.neighbors((1, 2)).collect::<Vec<_>>(), vec![(0, 2), (1, 3), (2, 2), (1, 1)]);
        assert_eq!(solv.neighbors((0, 2)).collect::<Vec<_>>(), vec![(1, 2)]);
        assert_eq!(solv.neighbors((3, 3)).collect::<Vec<_>>(), vec![(3, 4)]);
        assert_eq!(solv.neighbors((3, 0)).collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn solve_book() {
        let arr = array![
            [0,0,0,0,0,0,0,0,0u8],
            [1,1,1,1,0,1,1,1,0u8],
            [0,1,0,1,0,1,0,1,0u8],
            [0,1,0,1,1,1,0,1,0u8],
            [0,1,0,0,0,0,0,1,0u8],
            [0,1,1,1,1,0,0,1,0u8],
            [0,1,0,0,1,1,1,1,1u8],
            [0,0,0,0,0,0,0,0,0u8],
        ];
        let adim = arr.shape();
        assert_eq!(adim, &[8, 9]);
        let mut solv = Solver1::new_rc(adim[0] as DimType, adim[1] as DimType).unwrap();
        for (c, v) in arr.indexed_iter() {
            if *v == 1 {
                solv.set_passage(c.0 as DimType, c.1 as DimType);
            }
        }
        assert_eq!(solv.is_valid(), true);
        assert_eq!(
            format!("{:?}", solv.solve(true).unwrap()), 
            format!("{:?}", (4, vec![(1,0),(1,1),(2,1),(3,1),(4,1),(5,1),(5,2),(5,3),(5,4),(6,4),(6,5),(6,6),(6,7),(6,8)])) 
        );
    }

    #[test]
    fn solve_cycle() {
        let arr = array![
            [0,0,0,0,0,0,0,0,0u8],
            [1,1,1,1,0,1,1,1,0u8],
            [0,1,0,1,0,1,0,1,0u8],
            [0,1,0,1,1,1,0,1,0u8],
            [0,1,0,1,0,0,0,1,0u8],
            [0,1,1,1,1,0,0,1,0u8],
            [0,1,0,0,1,1,1,1,1u8],
            [0,0,0,0,0,0,0,0,0u8],
        ];
        let adim = arr.shape();
        assert_eq!(adim, &[8, 9]);
        let mut solv = Solver1::new_rc(adim[0] as DimType, adim[1] as DimType).unwrap();
        for (c, v) in arr.indexed_iter() {
            if *v == 1 {
                solv.set_passage(c.0 as DimType, c.1 as DimType);
            }
        }
        assert_eq!(solv.is_valid(), true);
        assert_eq!(solv.solve(false), Some((4, VecDeque::new())) );
    }

    #[test]
    fn solve_no_solution() {
        let arr = array![
            [0,0,0,0,0,0,0,0,0u8],
            [1,1,1,1,0,1,1,1,0u8],
            [0,1,0,1,0,1,0,1,0u8],
            [0,1,0,1,1,1,0,1,0u8],
            [0,1,0,1,0,0,0,0,0u8],
            [0,1,1,1,0,0,0,1,0u8],
            [0,1,0,0,1,1,1,1,1u8],
            [0,0,0,0,0,0,0,0,0u8],
        ];
        let adim = arr.shape();
        assert_eq!(adim, &[8, 9]);
        let mut solv = Solver1::new_rc(adim[0] as DimType, adim[1] as DimType).unwrap();
        for (c, v) in arr.indexed_iter() {
            if *v == 1 {
                solv.set_passage(c.0 as DimType, c.1 as DimType);
            }
        }
        assert_eq!(solv.is_valid(), true);
        assert_eq!(solv.solve(false), None );
    }
    
    #[test]
    fn rand_check() {
        let mut rng = rand::thread_rng();
        let r_dim:DimType = rng.gen_range(20, 40);
        let c_dim:DimType = rng.gen_range(20, 40);
        //println!("r_dim {}, c_dim {}", r_dim, c_dim);
        let r_dim_u = r_dim as usize;
        let c_dim_u = c_dim as usize;
        for _ in 1..10 {
            let arr = Array::random((r_dim_u, c_dim_u), Uniform::new_inclusive(0, 100));
            let mut solv = Solver1::new_rc(r_dim, c_dim).unwrap();
            init_from_rand_array(&mut solv, &arr.view(), 80);
            //println!("arr = \n{:3}", arr);
            if let Some(solution) = solv.solve(true) {
                // println!("solution  {:?}", solution);
                let mut test_done = false;
                for _ in 0..10 {
                    let r_test:DimType = rng.gen_range(5, r_dim - 5);
                    let c_test:DimType = rng.gen_range(5, c_dim - 5);
                    let r_test_u = r_test as usize;
                    let c_test_u = c_test as usize;
                    if !solv.is_passage(r_test - 1, c_test - 1) { continue; }
                    if solution.1.contains(&(r_test - 1, c_test - 1)) { continue; }
                    
                    let mut solv1 = Solver1::new_rc(r_test + 1, c_test).unwrap();
                    init_from_rand_array(&mut solv1, &arr.slice(s![..r_test_u+1, ..c_test_u]), 80);
                    //println!("r_test {}, c_test {}", r_test, c_test);
                    //println!("arr1 = \n{:3}", arr.slice(s![..r_test_u+1, ..c_test_u]));
                    let solution1 = solv1.solve(true);
                    if solution1.is_none() { continue; }
                    let mut solution1 = solution1.unwrap();
                    // println!("solution1 {:?}", solution1);
                    let node1b = solution1.1.pop_back().unwrap();
                    let node1bb = solution1.1.pop_back().unwrap();
                    
                    let mut solv2 = Solver1::new_rc(r_dim - r_test + 2, c_dim - c_test + 1).unwrap();
                    init_from_rand_array(&mut solv2, &arr.slice(s![r_test_u-2.., c_test_u-1..]), 80);
                    //println!("arr2 = \n{:3}", arr.slice(s![r_test_u-2.., c_test_u-1..]));
                    let solution2 = solv2.solve(true);
                    if solution2.is_none() { continue; }
                    let mut solution2 = solution2.unwrap();
                    // println!("solution2 {:?}", solution2);
                    let node2f = solution2.1.pop_front().unwrap();
                    let node2ff = solution2.1.pop_front().unwrap();

                    assert_eq!(node1b, (r_test - 1, c_test - 1));
                    
                    let turn_at_test = 
                        if node1b.0 == node1bb.0 && node2f.0 == node2ff.0 { 0 }
                        else if node1b.1 == node1bb.1 && node2f.1 == node2ff.1 { 0 }
                        else { 1 };
                    
                    // Check is optimal path has no more turns than path thru random node
                    assert!(solution.0 <= solution1.0 + solution2.0 + turn_at_test);
                    test_done = true;
                    break;
                }
                if test_done { break; }
            }
        }
    }
}
