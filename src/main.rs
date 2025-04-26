use library::{Internable, Interned, Label};
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<Interned<Label>, &'static str>::new();
    map.insert(Label("a").intern(), "This is a value");
    map.insert(Label("b").intern(), "This is another value");
    map.insert(Label("c").intern(), "So many values");

    for (key, value) in map {
        println!("{key:?}: {value}");
    }
}
