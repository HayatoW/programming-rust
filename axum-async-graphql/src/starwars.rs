use std::collections::HashMap;

use slab::Slab;

mod model;

pub struct StarWarsChar {
    id: &'static str,
    name: &'static str,
    is_human: bool,
}

pub struct StarWars {
    luke: usize,
    artoo: usize,
    chars: Slab<StarWarsChar>,
    chars_by_id: HashMap<&'static str, usize>,
}
