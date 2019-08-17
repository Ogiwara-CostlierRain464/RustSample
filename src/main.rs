use std::fmt;

static mut par: [usize;10] = [0,1,2,3,4,5,6,7,8,9];
static mut rank: [usize;10] = [0;10];

fn main(){
    unsafe {
        test();
    }
}

// 木の根を求める
unsafe fn find(x: usize) -> usize{
    if par[x] == x {
        return x;
    }else{
        par[x] = find(par[x]);
        return par[x];
    }
}

// xとyの属する集合を併合
unsafe fn unite(mut x: usize,mut y: usize){
    x = find(x);
    y = find(y);
    if x == y {return;}

    if rank[x] < rank[y] {
        par[x] = y;
    }else{
        par[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

// xとyが同じ集合に属するか否か
unsafe fn same(x: usize, y: usize) -> bool{
    return find(x) == find(y);
}

unsafe fn test(){
    unite(1,2);
    unite(1,5);

    unite(6,3);
    unite(4,7);
    unite(6,7);

    println!("{}", same(6, 2));
}