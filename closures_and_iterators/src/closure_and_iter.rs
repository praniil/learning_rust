#[derive(PartialEq, Debug)]
struct Shoe{
    size : u32,
    style : String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let result: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == shoe_size).collect();
    result
}

pub fn closure_and_iter() {
    let shoes = vec![
        Shoe{
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let available_shoes: Vec<Shoe> = shoes_in_size(shoes, 10);
    println!("{:#?}", available_shoes);
}