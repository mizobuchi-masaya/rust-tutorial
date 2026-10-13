use averaged_collection::AveragedCollection;

fn main() {
    let mut c = AveragedCollection::new();

    c.add(10);
    c.add(30);

    println!("c.average = {:?}", c.average());
}
