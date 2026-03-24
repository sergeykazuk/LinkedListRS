mod double_linked_list;

fn main() {

    let mut dl = double_linked_list::DoubleLinkedList::<i16>::create();
    dl.add_value(&3);
    dl.add_value(&12);
    dl.add_value(&45);
    dl.add_value(&24);
    dl.print_all();
    dl.print_all_backwards();

    dl.remove_value(&3);
    dl.print_all();

    dl.remove_value(&45);
    dl.print_all();

    dl.add_value(&15);
    dl.add_value(&52);

    dl.print_all();

    let mut element_at = dl.at(3);
    println!("Element at position 3: {}", element_at.unwrap_or(i16::MIN));
}
