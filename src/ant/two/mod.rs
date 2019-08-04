pub mod maze;

pub fn leaf(a: &[i32],n: i32, k: i32){
    println!("{}", dfs(0,0,a,n,k));
}

fn dfs(i: i32, sum: i32, a: &[i32], n: i32, k:i32) -> bool{
    if i == n {return sum == k}
    if dfs(i + 1, sum, a, n, k) {return true}
    if dfs(i + 1, sum + a[i as usize], a, n, k){return true}

    false
}