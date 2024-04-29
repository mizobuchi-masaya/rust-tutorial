fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let month = ["January", "February", "March", "April", "May", "June", "July",
                 "August", "September", "October", "November", "December"];
    println!("{:?}", month);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", b);

    let c = [3; 5]; 
    println!("{:?}", c);
}
