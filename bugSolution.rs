fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of manipulating raw pointers, use safe methods to modify vector contents:
    v[0] = 10; //Direct indexing
    println!("v: {:?}", v);

    //or
    let index = 0;
    *v.get_mut(index).unwrap() = 10; //Safe get_mut method which returns an Option
    println!("v: {:?}", v);
} 