use std::vec;

#[derive(Clone)]
pub struct Item {
    pub cost: usize,
    pub stat: usize,
}

impl Item {
    pub fn new(cost: usize, stat: usize) -> Item {
        Item { cost, stat }
    }
}

pub struct Shop {
    pub weapons: Vec<Item>,
    pub armour: Vec<Item>,
    pub damage_rings: Vec<Item>,
    pub armour_rings: Vec<Item>,
}

impl Shop {
    pub fn new() -> Shop {
        let blank = Item::new(0, 0);
        Shop {
            weapons: vec![
                Item::new(8, 4),
                Item::new(10, 5),
                Item::new(25, 6),
                Item::new(40, 7),
                Item::new(74, 8),
            ],
            armour: vec![
                blank.clone(),
                Item::new(13, 1),
                Item::new(31, 2),
                Item::new(53, 3),
                Item::new(75, 4),
                Item::new(102, 5),
            ],
            damage_rings: vec![
                blank.clone(),
                Item::new(25, 1),
                Item::new(50, 2),
                Item::new(100, 3),
            ],
            armour_rings: vec![
                blank.clone(),
                Item::new(20, 1),
                Item::new(40, 2),
                Item::new(80, 3),
            ],
        }
    }
}
