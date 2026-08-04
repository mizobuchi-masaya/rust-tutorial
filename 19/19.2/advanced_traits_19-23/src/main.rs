fn main() {
    use std::ops::Add;

    #[derive(Debug)]
    struct Millimeters(u32);
    #[derive(Debug)]
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let mm = Millimeters(923);
    let m = Meters(1);

    println!("{:?} {:?}", m, mm);
    println!(" = {:?}", mm + m);
}
