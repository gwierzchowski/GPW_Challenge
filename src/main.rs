use std::env;
use std::io;
use std::collections::VecDeque;

mod dijkstra_speed;
mod dijkstra_mem;

fn solver_factory(rows: DimType, cols: DimType) -> Result<impl Solver, &'static str> {
    dijkstra_speed::Solver1::new_rc(rows, cols)
    //dijkstra_mem::Solver1::new_rc(rows, cols)
}

fn main() -> Result<(), isize> {
    let args: Vec<String> = env::args().collect();
    let mut arg_dbg = false;
    let mut arg_converter = false;
    if args.len() > 1 {
        arg_dbg = args[1] == "--dbg";
        arg_converter = !arg_dbg;
    }

    let stdin = io::stdin();
    let mut buf = String::new();
    if let Err(e) = stdin.read_line(&mut buf) {
        eprintln!("Input line 1: {}", e);
        return Err(-2);
    }
    #[allow(non_snake_case)]
    let XY: Vec<_> = buf.split(',')
        .map(|s| DimType::from_str_radix(s.trim(), 10))
        .map(|n| n.map_err(|e| {eprintln!("Input line 1: parsing error: {}", e); -2isize}))
        .collect();
    if XY.len() < 2 {
        eprintln!("Input line 1: should have 2 comma separated elements");
        return Err(-2);
    }
    #[allow(non_snake_case)]
    let (X, Y) = (XY[0]?, XY[1]?);

    if arg_converter {
        use num::bigint::BigUint;
        use num_traits::Num;

        buf.clear();
        let mut line_no = 2;
        while let Ok(n) = stdin.read_line(&mut buf) {
            if n > 0 {
                match BigUint::from_str_radix(buf.trim_end(), 2) {
                    Ok(num) => {
                        println!("{}", num);
                    },
                    Err(e) => {
                        eprintln!("Line {} - parsing error: {}", line_no, e);
                    }
                }
            } else { break }
            buf.clear();
            line_no += 1;
        }
        Ok(())
    } else {
        let mut solver = match solver_factory(Y, X) {
            Ok(solver) => solver,
            Err(msg) => {
                eprintln!("Could not create solver: {}", msg);
                return Err(-2);
            }
        };

        for line_no in 0..Y {
            buf.clear();
            if let Err(e) = stdin.read_line(&mut buf) {
                eprintln!("Input line {}: {}", line_no + 2, e);
                return Err(-2);
            }
            let line = buf.trim_end();
            if line.len() != X as usize {
                eprintln!("Input line {}: line should have {} characters, but has {}", line_no + 2, X, line.len());
                return Err(-2);
            }
            let mut ch_no:DimType = 0;
            for ch in line.chars() {
                match ch {
                    '0' => {},
                    '1' => {
                        solver.set_passage(line_no, ch_no);
                    },
                    _ => {
                        eprintln!("Input line {}: invalid character: {}", line_no + 2, ch);
                        return Err(-2);
                    }
                }
                ch_no += 1;
            }

        }
        if !solver.is_valid() {
            return Err(-1);
        }

        if let Some((result, path)) = solver.solve(arg_dbg) {
            println!("{}", result);
            if arg_dbg {
                println!("{:?}", path);
            }
            Ok(())
        } else {
            Err(-1)
        }
    }
}

/// To store dimension of puzzle
/// must be castable to usize
type DimType = u16;

type NodeAdr = (DimType, DimType);

// Sized is required in order to provide default implementation of solve_and_drop
pub trait Solver:Sized {
    fn set_passage(&mut self, r: DimType, c:DimType);
    fn is_valid(&self) -> bool;
    fn solve(&self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)>;
    fn solve_and_drop(self, with_path: bool) -> Option<(DimType, VecDeque<NodeAdr>)> {
        self.solve(with_path)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Tests
/// 

#[cfg(test)]
mod tests {
    use rand::Rng;
    use ndarray::{Array, ArrayView2};
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand_distr::Uniform;
    use super::*;
    
    pub fn init_from_rand_array<A:PartialOrd>(solv:&mut impl Solver, arr:&ArrayView2<A>, frac:A) {
        let dim = arr.shape();
        for (c, v) in arr.indexed_iter() {
            if *v <= frac {
                solv.set_passage(c.0 as DimType, c.1 as DimType);
            }
        }
        solv.set_passage(1, 0);
        solv.set_passage(dim[0] as DimType - 2, dim[1] as DimType - 1);
        assert!(solv.is_valid());
    }

    #[test]
    fn size_constraints() {
        assert!(DimType::MAX >= 10_000);
        assert!((DimType::MAX as usize)^2 <= usize::MAX);
        assert_eq!(dijkstra_speed::Solver1::new_rc(10_000, 10_000).is_ok(), true);
        assert_eq!(dijkstra_mem::Solver1::new_rc(10_000, 10_000).is_ok(), true);
    }
    
    #[test]
    fn cross_check() {
        let mut rng = rand::thread_rng();
        let r_dim = rng.gen_range(5, 20);
        let c_dim = rng.gen_range(5, 20);
        let arr = Array::random((r_dim, c_dim), Uniform::new_inclusive(0, 100));
        let mut solv1 = dijkstra_speed::Solver1::new_rc(r_dim as DimType, c_dim as DimType).unwrap();
        let mut solv2 = dijkstra_mem::Solver1::new_rc(r_dim as DimType, c_dim as DimType).unwrap();
        init_from_rand_array(&mut solv1, &arr.view(), 75);
        init_from_rand_array(&mut solv2, &arr.view(), 75);
        assert_eq!(solv1.solve(false), solv2.solve(false));
    }
    
}
