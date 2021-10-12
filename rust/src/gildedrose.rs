use std::fmt::{self, Display};
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            match item.name.as_str() {
                x if x.contains("Aged Brie") => update_aged_brie(item),
                x if x.contains("Backstage passes") => update_backstage_passes(item),
                x if x.contains("Conjured") => update_others(item, 2),
                x if x.contains("Sulfuras") => (),
                _ => update_others(item, 1),
            }
            if !item.name.contains("Sulfuras"){
                item.sell_in = item.sell_in - 1
            }
        }
    }
}

fn update_others(item: &mut Item, update: i32) {
    match item.quality {
        quality if quality > 0 && quality < 50 => {
            if item.sell_in > 0 {
                item.quality = item.quality - update
            } else {
                item.quality = item.quality - 2*update
            }
        }
        _ => return,
    }
}

fn update_aged_brie(item: &mut Item) {
    match (item.quality, item.sell_in) {
        (quality, sell_in) if quality < 50 && sell_in > 0 => item.quality = item.quality + 1,
        (quality, sell_in) if quality < 50 && sell_in <= 0 => item.quality = item.quality + 2,
        _ => return,
    }
}

fn update_backstage_passes(item: &mut Item) {
    match (item.quality, item.sell_in) {
        (quality, sell_in) if quality < 48 && sell_in > 0 && sell_in < 6  => item.quality  = item.quality + 3,
        (quality, sell_in) if quality < 49 && sell_in > 0 &&  sell_in < 11 => item.quality = item.quality + 2,
        (quality, sell_in) if quality < 50 && sell_in > 0 => item.quality = item.quality + 1,
        (quality, sell_in) if quality == 50 && sell_in > 0 => (),
        _ =>  item.quality = 0
    };
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

    #[test]
    pub fn test_generic_items() {
        let items = vec![
            Item::new("+5 Dexterity Vest", 10, 20),
            Item::new("Elixir of the Mongoose", 0, 7),
            Item::new("Elixir of the Mongoose", 6, 0),
        ];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("+5 Dexterity Vest", rose.items[0].name);
        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(19, rose.items[0].quality);

        assert_eq!("Elixir of the Mongoose", rose.items[1].name);
        assert_eq!(-1, rose.items[1].sell_in);
        assert_eq!(5, rose.items[1].quality);

        assert_eq!("Elixir of the Mongoose", rose.items[2].name);
        assert_eq!(5, rose.items[2].sell_in);
        assert_eq!(0, rose.items[2].quality);
    }

    #[test]
    pub fn test_aged_brie_items() {
        let items = vec![
            Item::new("Aged Brie", 10, 10),
            Item::new("Aged Brie", 10, 50),
            Item::new("Aged Brie", -2, 32),
            Item::new("Aged Brie", 10, 52),
            Item::new("Aged Brie", 2, 0),
        ];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("Aged Brie", rose.items[0].name);
        assert_eq!(9, rose.items[0].sell_in);
        assert_eq!(11, rose.items[0].quality);

        assert_eq!("Aged Brie", rose.items[1].name);
        assert_eq!(9, rose.items[1].sell_in);
        assert_eq!(50, rose.items[1].quality);

        assert_eq!("Aged Brie", rose.items[2].name);
        assert_eq!(-3, rose.items[2].sell_in);
        assert_eq!(34, rose.items[2].quality);

        assert_eq!("Aged Brie", rose.items[3].name);
        assert_eq!(9, rose.items[3].sell_in);
        assert_eq!(52, rose.items[3].quality);

        assert_eq!("Aged Brie", rose.items[4].name);
        assert_eq!(1, rose.items[4].sell_in);
        assert_eq!(1, rose.items[4].quality);
    }

    #[test]
    pub fn test_sulfuras_items() {
        let items = vec![
            Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
            Item::new("Sulfuras, Hand of Ragnaros", -1, 80),
            Item::new("Sulfuras, Hand of Ragnaros", 30, 60),
        ];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("Sulfuras, Hand of Ragnaros", rose.items[0].name);
        assert_eq!(0, rose.items[0].sell_in);
        assert_eq!(80, rose.items[0].quality);

        assert_eq!("Sulfuras, Hand of Ragnaros", rose.items[1].name);
        assert_eq!(-1, rose.items[1].sell_in);
        assert_eq!(80, rose.items[1].quality);

        assert_eq!("Sulfuras, Hand of Ragnaros", rose.items[2].name);
        assert_eq!(30, rose.items[2].sell_in);
        assert_eq!(60, rose.items[2].quality);
    }

    #[test]
    pub fn test_backstage_passes_items() {
        let items = vec![
            Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 42),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 23),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 23),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 4, 48),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 8, 49),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 12, 50),
        ];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[0].name
        );
        assert_eq!(14, rose.items[0].sell_in);
        assert_eq!(21, rose.items[0].quality);

        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[1].name
        );
        assert_eq!(9, rose.items[1].sell_in);
        assert_eq!(44, rose.items[1].quality);

        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[2].name
        );
        assert_eq!(4, rose.items[2].sell_in);
        assert_eq!(26, rose.items[2].quality);

        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[3].name
        );
        assert_eq!(-1, rose.items[3].sell_in);
        assert_eq!(0, rose.items[3].quality);

        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[4].name
        );
        assert_eq!(3, rose.items[4].sell_in);
        assert_eq!(50, rose.items[4].quality);

        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[5].name
        );
        assert_eq!(7, rose.items[5].sell_in);
        assert_eq!(50, rose.items[5].quality);

        assert_eq!(
            "Backstage passes to a TAFKAL80ETC concert",
            rose.items[6].name
        );
        assert_eq!(11, rose.items[6].sell_in);
        assert_eq!(50, rose.items[6].quality);
    }

    #[test]
    pub fn test_conjured_items() {
        let items = vec![
            Item::new("Conjured Mana Cake", 3, 6),
            Item::new("Conjured Mana Cake", 0, 10),
            Item::new("Conjured Mana Cake", 6, 0),
        ];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("Conjured Mana Cake", rose.items[0].name);
        assert_eq!(2, rose.items[0].sell_in);
        assert_eq!(4, rose.items[0].quality);

        assert_eq!("Conjured Mana Cake", rose.items[1].name);
        assert_eq!(-1, rose.items[1].sell_in);
        assert_eq!(6, rose.items[1].quality);

        assert_eq!("Conjured Mana Cake", rose.items[2].name);
        assert_eq!(5, rose.items[2].sell_in);
        assert_eq!(0, rose.items[2].quality);
    }
}
