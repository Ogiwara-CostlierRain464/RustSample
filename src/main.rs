use std::fmt::Display;

fn long<'a, T: Display>(x: &'a str, y: &'a str, ann :T) -> &'a str{
    println!("ann {}", ann);
    if  x.len() > y.len(){
        x
    } else{
        y
    }
}
fn main(){

}