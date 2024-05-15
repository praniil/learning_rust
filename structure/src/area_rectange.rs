#[derive(Debug)]
struct Rectange {
    length: u32,
    width: u32,
}

pub fn area_rectange() {
    //first method
    let length: u32 = 50;
    let breadth : u32= 100;
    let area : u32 = area_of_rect(length, breadth);
    println!("the area of rectange is : {}", area);

    //using tuples
    //better way because it is more readable 
    //but in another way this version is less clear: tuples dont use the name of the item inside it
    //we dont know which one is length and which one is the breadth
    let rectange : (u64, u64) = (50, 100);
    println!("the are of rectange using tuble: {}", area_using_tuple(rectange));

    //using structures which is even a better way
    let rect1 : Rectange = Rectange{length: 50, width: 100};
    println!("rect1 is: {:#?}", rect1);
    println!("the are of rectange using struct is: {}", area_using_struct(&rect1));
}

fn area_of_rect(lenth: u32, breadth: u32) -> u32 {
    return lenth * breadth;
}

fn area_using_tuple(dimension: (u64, u64)) -> u64 {
    return dimension.0 * dimension.1;
}

fn area_using_struct(rect: &Rectange) -> u32 {
    rect.length * rect.width
}