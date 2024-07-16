
fn largest_number(number_list: &[i32]) -> &i32 {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
pub fn test4_1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = largest_number(&number_list);

    println!("The largest number is {largest}");
    println!("number_list is : {number_list:?}")
}

pub fn test4_2() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let res = largest(&number_list);
    println!("number_list - res : {res}");

    let res = largest(&char_list);
    println!("char_list - res : {res}");
}


