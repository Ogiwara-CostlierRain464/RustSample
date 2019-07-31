use std::cmp::{min, max};

/// アリが与えられて、アリが落ちるまでの最小と最大の方向
/// アリの初期の位置はわからない
pub fn ants (l: i32, ants: &[i32]){
    let mut min_t = 0;
    for &ant_pos in ants{
        min_t = max(min_t, min(ant_pos, l - ant_pos));
    }

    let mut max_t = 0;
    for &ant_pos in ants{
        max_t = max(max_t, max(ant_pos, l - ant_pos));
    }

    println!("min: {} max: {}", min_t, max_t);
}
