fn main() {
    let v = vec![1, 2, 3, 4, 5];

//    let does_not_exist = &v[100];
//    println!("The third element is {does_not_exist}");

    let does_not_exist = v.get(100);
    println!("The third element is {:?}", does_not_exist);
}
