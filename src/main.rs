use std::io;
use tower_of_hanoi::solve;

fn main() {
   let mut input = String::new();
   io::stdin().read_line(&mut input).unwrap();

   let n: u32 = input.trim().parse().unwrap();

   let mut moves: Vec<(u32, u32)> = Vec::new();

   solve(n, 1, 3, 2, &mut moves);
   
   println!("{}", moves.len());

   for (from, to) in moves {
         println!("{} {}", from, to);
   }
}

