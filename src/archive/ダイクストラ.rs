use std::cmp::{ min };

fn main(){
    dijkstra();
}

const I: usize = 10000;
const V: usize = 7;

fn dijkstra(){
    let cost = [
        [0, 5, 2, I, I, I, I],
        [5, 0, 4, 2, I, I, I],
        [2, 4, 0, 6,10, I, I],
        [I, 2, 6, 0, I, 1, I],
        [I, I,10, I, 0, 3, 5],
        [I, I, I, 1, 3, 0, 9],
        [I, I, I, I, 5, 9, 0],
    ];
    let mut d = [I;V];
    let mut used = [false;V];
    d[0] = 0;

    loop {
        let mut v: isize = -1;
        for u in 0..V{
            if !used[u] && (v == -1 || d[u] < d[v as usize]){
                v = u as isize;
            }
            println!("v is {}", v);
        }
        if v == -1{break;}
        let v = v as usize;
        used[v as usize] = true;
        for u in 0..V{
            d[u] = min(d[u], d[v] + cost[v][u]);
        }
        println!("used is {:?}, d is {:?}", used, d);
    }

    println!("{:?}",d);
}