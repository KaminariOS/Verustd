spec fn min(x: int, y: int) -> int {
    if x < y {
        x
    } else {
        y
    }
}

fn test() {
    assert(min(1, 2) ==1);
    assert(min(100, 200) == 100);
}