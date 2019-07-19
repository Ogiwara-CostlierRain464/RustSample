fn main(){
    let str = String::from("long");
    let result;

    {
        let str2 = String::from("xyz");
        result = longest(str.as_str(), str2.as_str());
    }

    print!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
