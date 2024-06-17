pub fn boomrang() {
    let any_vector: Vec<i32> = vec![3, 7, 3, 2, 1, 5, 1, 2, 2, -2, 2, -3, 4, -3];
    let any_vector_iter = any_vector.iter();
    let mut count = 0;
    for(index, _value) in any_vector_iter.enumerate() {
        if index <= any_vector.len() - 3 {
            if  any_vector[index] == any_vector[index + 2]{
                count = count + 1;
            } 
        } else{
            break;
        }
    }
    println!("count : {}", count);
}