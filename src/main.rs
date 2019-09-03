//#[macro_use]
//extern crate lazy_static;

fn main(){
    println!("{}", sieve(100000));
}

// n以下の素数の個数
fn sieve(n: usize) -> usize{
    let mut prime = [0;200000];
    let mut is_prime = [false;200001];

    let mut p = 0;
    for i in 0..n{
        is_prime[i] = true;
    }
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n{
        if is_prime[i]{
            p += 1;
            prime[p] = i;
            for j in (2*i..n).step_by(i){
                is_prime[j] = false;
            }
        }
    }

    for &e in prime.iter(){
        if e != 0{
            print!("{},", e);
        }
    }

    return p;
}