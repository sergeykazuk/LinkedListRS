
mod single_linked_list;


fn main() {

    let mut ll = single_linked_list::SingleLinkedList::<i16>::new();

    ll.print_all();

    println!("\nAdding values...");
    ll.add_value(&14);
    ll.add_value(&25);
    ll.add_value(&32);
    ll.add_value(&59);

    ll.print_all();
    println!("List size: {}\n", ll.size());

    println!("Removing some values...");
    ll.remove_value(&1);
    ll.remove_value(&25);

    ll.print_all();

    println!("Removing remaining values...");
    ll.remove_value(&14);

    ll.print_all();

    ll.remove_value(&59);

    ll.print_all();

}
