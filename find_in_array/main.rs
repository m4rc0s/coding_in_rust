fn main() {
    println!("Hello, world!");

    let elements = [1, 2, 3, 4 , 5];

    let res = find_element(&elements, &100);

    println!("The result is {:?}", res);

}

fn find_element(elements: &[i32], desired_element: &i32) -> i32 {
    for e in elements {
        println!("{:?}", e);

        if e == desired_element { return *e; }
    }

    return 0i32;
}
