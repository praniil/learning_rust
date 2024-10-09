fn large<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn generics() {
    let list1 = vec![2, 3, 4, 5];
    let largest1 = large(&list1);
    println!("largest: {largest1}");

    let list_char = vec!['a', 'b', 'c', 'd'];
    let largest_char = large(&list_char);
    println!("largest char: {largest_char}");
}