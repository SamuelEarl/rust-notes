// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    qty: i32,
    id: i32,
}

fn display_qty(item: &GroceryItem) {
    println!("QTY: {0}", item.qty);
}

fn display_id(item: &GroceryItem) {
    println!("ID: {0}", item.id);
}

fn main() {
    let item: GroceryItem = GroceryItem { qty: 5, id: 1};
    display_qty(&item);
    display_id(&item);
}
