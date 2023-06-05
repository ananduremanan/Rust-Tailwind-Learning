use std::collections::HashMap;

struct Store {
    id: u32,
    name: String,
}

impl Store {
    fn new(id: u32, name: String) -> Store {
        Store { id, name }
    }

    fn add(&self, grocery_list: &mut HashMap<u32, String>) {
        grocery_list.insert(self.id, self.name.clone());
        println!("Grocery item added!");
    }

    fn delete(&self, grocery_list: &mut HashMap<u32, String>) {
        grocery_list.remove(&self.id);
        println!("Grocery item removed!");
    }

    fn display(&self, grocery_list: &HashMap<u32, String>) {
        println!("Grocery List: {:?}", grocery_list);
    }
}

fn main() {
    let mut grocery_list: HashMap<u32, String> = HashMap::new();
    
    let gr_item1 = Store::new(1, "Apple".to_string());
    let gr_item2 = Store::new(2, "Wheat".to_string());

    gr_item1.add(&mut grocery_list);
    gr_item2.add(&mut grocery_list);

    gr_item1.display(&grocery_list);
    gr_item2.display(&grocery_list);

    gr_item1.delete(&mut grocery_list);

    gr_item1.display(&grocery_list);
    gr_item2.display(&grocery_list);
}
