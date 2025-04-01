spec fn min(x: int, y: int) -> int {
    if x < y {
        x
    } else {
        y
    }
}

spec fn min3(x:int, y:int, z: int) -> int {
    min(x, min(y, z))
}

fn compute_min3(x: u64, y: u64, z:u64) -> (m: u64)
    ensures
        m == min3(x as int, y as int, z as int),
{   
    let mut m = x;
    if y < m {
        m = y;
    }
    if z < m {
        m = z;
    }
    m
}

fn test() {
    let m = compute_min3(1, 2, 3);
    assert(m == 1);
    let m = compute_min3(100, 200, 300);
    assert(m == 100);
}