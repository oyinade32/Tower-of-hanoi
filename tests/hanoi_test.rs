use tower_of_hanoi::solve_recursive;

#[test]
fn test_hanoi_n_1() {

    let mut moves = Vec::new();
    solve_recursive(1, 1, 3, 2, &mut moves);

    assert_eq!(moves.len(), 1);

}


#[test]

fn test_hanoi_n_2() {

    let mut moves = Vec::new();
    solve_recursive(2, 1, 3, 2, &mut moves);

    assert_eq!(moves.len(), 3);

}

#[test]

fn test_hanoi_n_3() {
    let mut moves = Vec::new();
    solve_recursive(3, 1, 3, 2, &mut moves);

    assert_eq!(moves.len(), 7);
}