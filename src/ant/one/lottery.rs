pub fn lotterry(arr: Vec<i32>, sum: i32){
    let mut result = false;

    for &a in &arr{
        for &b in &arr{
            for &c in &arr{
                for &d in &arr{
                    if a+b+c+d == sum{
                        result = true;
                    }
                }
            }
        }
    }

    println!("{}", result);
}