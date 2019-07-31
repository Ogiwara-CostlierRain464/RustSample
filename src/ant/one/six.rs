/// n本のa_iの長さの棒が与えられる
/// ここから3本選んで、最大の周長をもとめよう
pub fn triangle(vec: &[i32]){
    let mut result = 0;

    for a in vec.iter(){
        for b in vec.iter().filter(|num| *num != a ){
           for c in vec.iter().filter(|num| *num != a && *num != b ){
                if can_make_triangle(*a,*b,*c){
                    result = a + b + c;
                }
            }
        }
    }

    println!("{}", result);
}

fn can_make_triangle(a: i32, b: i32, c: i32) -> bool{
    let arr = [a,b,c];
    let max: &i32 = arr.iter().max().unwrap();

    let sum_of_rest: i32 = arr.iter().filter(|num| *num != max).sum();

    sum_of_rest > *max
}