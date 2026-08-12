fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and fielld_value are invalid at this point, try using them and
    // see what compiler error you get!

    dbg!(map);
    dbg!(field_name);
    dbg!(field_value);
}
