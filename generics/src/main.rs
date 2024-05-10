use std::cmp::Ordering;


fn min<T: Ord> (l: T, r:T) -> T {
    match l.cmp(&r) {
        Ordering::Less | Ordering::Equal => l,
        Ordering::Greater => r 
    }
} 

fn main() {
    println!("this is the min number: {:?}", min(243, 97));
}
