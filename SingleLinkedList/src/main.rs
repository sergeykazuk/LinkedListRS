
mod single_linked_list;


fn main() {

    let mut ll = single_linked_list::SingleLinkedList::<i16>::new();

    ll.print_all();

    println!("\nAdding values...");
    ll.push_back(&14);
    ll.push_back(&25);
    ll.push_back(&32);
    ll.push_back(&59);
    ll.push_back(&88);

    println!("List size: {}\n", ll.size());
    ll.print_all();

    println!("Popping...");
    ll.pop_back();
    ll.pop_front();
    println!("List size: {}\n", ll.size());
    ll.print_all();

    ll.push_back(&31);
    ll.push_front(&17);

    println!("Removing some values...");
    ll.remove_value(&1);
    ll.remove_value(&25);

    ll.print_all();

    println!("Removing remaining values...");
    ll.remove_value(&14);

    ll.print_all();

    ll.remove_value(&59);

    ll.print_all();

    ll.push_front(&99);

    ll.print_all();

}
