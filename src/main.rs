use std::cmp::max;

const N: usize = 4;
const ARR: [(usize,usize);4] = [(2,3),(1,2),(3,4),(2,2)];
const W: usize = 5;

const w: [usize;4] = [2,1,3,2];
const v: [usize;4] = [3,2,4,2];

fn main(){
    unsafe {
       // println!("{}", dfss(0, W));
       dfsss();
    }
}

/// i: current index
/// acc: (weight, value)
/// 
fn dfs(i: usize, acc: (usize, usize)) -> usize{
    if acc.0 > W{ return 0; }
    if i == N{ return acc.1; }

    return max(
        dfs(i + 1, acc),
        dfs(i + 1, (acc.0 + ARR[i].0, acc.1 + ARR[i].1)),
        )
    
}

static mut DP: [[usize;N + 2];N + 2] = [[0;N + 2];N + 2];

/// DPのために変更！
/// (index, least weight)
/// 
unsafe fn dfss(i: usize, least_weight: usize) -> usize{
    if DP[i][least_weight] != 0{
        return DP[i][least_weight];
    }

    let mut res = 0;

    if i == N {
        res = 0; 
    }else if ARR[i].0 > least_weight { 
        res = 0; 
    }else{
        res =  max(
            dfss(i + 1, least_weight),
            dfss(i + 1, least_weight - ARR[i].0) + ARR[i].1
        );
    }

    DP[i][least_weight] = res;

    return res;
}

/// |- dp[n][j] = 0
/// |- dp[i][j] = |- dp[i+1][j] when j < w[i]
///               |- max(
///                     dp[i+1][j], 
///                     dp[i+1][j-w[i]]+v[i]
///                  ) when otherwise
unsafe fn dfsss(){
    for i in (0..N).rev(){
        for j in 0..=W{
            if j < w[i]{
                DP[i][j] = DP[i + 1][j];
            }else{
                DP[i][j] = max(DP[i + 1][j], DP[i + 1][j - w[i]] + v[i]);
            }
        }
    }
    println!("{}", DP[0][W]);
}