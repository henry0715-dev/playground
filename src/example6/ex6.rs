
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test() {
    let string1 =  String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {result}");
    println!("{} {}", string1, string2);
}