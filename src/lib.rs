pub fn solve(n: u32, from: u32, to: u32, helper: u32, moves: &mut Vec<(u32, u32)>) {
    if n == 1 {
        moves.push((from, to));
        return;
    }

    solve(n - 1, from, helper, to, moves);
    moves.push((from, to));
    solve(n - 1, helper, to, from, moves);
}