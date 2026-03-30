pub fn solve(n: i32, from: i32, to: i32, helper: i32, moves: &mut Vec<(i32, i32)>) {
    if n == 1 {
        moves.push((from, to));
        return;
    }

    solve(n - 1, from, helper, to, moves);
    moves.push((from, to));
    solve(n - 1, helper, to, from, moves);
}