//pub mod maze;

use std::cmp::{min, max};

#[allow(dead_code)]
fn leaf(a: &[i32],n: i32, k: i32){
    println!("{}", dfs(0,0,a,n,k));
}

#[allow(dead_code)]
fn dfs(i: i32, sum: i32, a: &[i32], n: i32, k:i32) -> bool{
    if i == n {return sum == k}
    if dfs(i + 1, sum, a, n, k) {return true}
    if dfs(i + 1, sum + a[i as usize], a, n, k){return true}

    false
}

// 貪欲法
#[allow(dead_code)]
pub fn coin(coins: &[usize;6], mut a: usize){
    const V: [usize;6] = [1,5,10,50,100,500];
    let mut ans = 0;

    for i in (0..=5).rev(){
        // コインiを使う枚数
        let t = min(a / V[i], coins[i]);
        println!("{}", i);
        a -= t * V[i];
        ans += t;
    }

    println!("{}", ans);
}

use std::mem;

#[allow(dead_code)]
pub fn fence(l: &mut [usize]){
    let mut n = l.len();
    let mut ans: u128 = 0;

    // 板が一本になるまで適用
    while n > 1 {
        let mut mii1 = 0; let mut mii2 = 1;
        if l[mii1] > l[mii2] { 
            mem::swap(&mut mii1, &mut mii2);
        }

        for i in 2..n{
            if l[i] < l[mii1] {
                mii2 = mii1;
                mii1 = i;
            }else if l[i] < l[mii2]{
                mii2 = i;
            }
        }

        let t = l[mii1] + l[mii2];
        ans += t as u128;

        if mii1 == n - 1 {
            mem::swap(&mut mii1, &mut mii2);
        }   

        l[mii1] = t;
        l[mii2] = l[n - 1];
        n -= 1;
    }

    println!("{:?}", l.to_vec());
}

// まずは全探査
// 再帰計算を用いて、全探索を行う
pub fn knapsack(){
    unsafe {
        println!("{}", rec(0, WEIGHT));
    }

}

static ARR: &[(usize, usize)] = &[(2,3), (1,2), (3,4), (2,2)];
const WEIGHT: usize = 5;

// i番目の要素を見ている時に、重さがjである、という散策が重複する。
static mut DP: [[usize;6];6] = [[INF;6];6];
const INF: usize = 1000;

/// i番目の品物を現在散策中  
/// 残り耐えられる重さはj
unsafe fn rec(i: usize, j: usize) -> usize{
    // すでにDPに記録済みなら
    if DP[i][j] != INF{
        return DP[i][j];
    }

    if i == ARR.len(){
        // もう品物は残ってない
        DP[i][j] = 0;
    }else if j < ARR[i].0{
        // この品物は入らないので、スキップ
        DP[i][j] = rec(i + 1, j);
    }else{
        DP[i][j] = max(rec(i+1,j),rec(i+1, j - ARR[i].0) + ARR[i].1);
    }

    DP[i][j]
}



// 車で遠足するやつ
pub fn expedition(){
    use std::collections::BinaryHeap;

    let L = 25;            // 距離
    let P = 10;            // ガソリンの量
    let mut N = 4;
    let mut A = [10,14,20,21,0]; // 距離
    let mut B = [10,5,2,4,0];    // 補給できるガソリン容量
    
    // 簡単のため、ゴールをガソリンスタンドに追加
    A[N] = L;
    B[N] = 0;
    N += 1;

    let mut queue: BinaryHeap<usize> = BinaryHeap::new();

    let mut ans = 0;
    let mut pos = 0;
    let mut tank = P;

    for i in 0..N{
        let d = A[i] - pos;

        while (tank as isize - d as isize) < 0 {
            if queue.is_empty() {
                panic!();
            }
            tank += queue.pop().unwrap();
            ans += 1;
        }
        
        tank -= d;
        pos = A[i];
        queue.push(B[i]);
    }

    println!("{}", ans);
}