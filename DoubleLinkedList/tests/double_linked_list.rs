

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use double_linked_list::DoubleLinkedList;

    #[test]
    fn test_add_and_at() {
        let mut list = DoubleLinkedList::create();
        list.add_value(&1);
        list.add_value(&2);
        list.add_value(&3);
        assert_eq!(list.size(), 3);
        assert_eq!(list.at(0), Some(1));
        assert_eq!(list.at(1), Some(2));
        assert_eq!(list.at(2), Some(3));
        assert_eq!(list.at(3), None);
    }

    #[test]
    fn test_remove_value() {
        let mut list = DoubleLinkedList::create();
        list.add_value(&10);
        list.add_value(&20);
        list.add_value(&30);
        list.remove_value(&20);
        assert_eq!(list.size(), 2);
        assert_eq!(list.at(0), Some(10));
        assert_eq!(list.at(1), Some(30));
        assert_eq!(list.at(2), None);
        list.remove_value(&10);
        assert_eq!(list.size(), 1);
        assert_eq!(list.at(0), Some(30));
        list.remove_value(&30);
        assert_eq!(list.size(), 0);
        assert_eq!(list.at(0), None);
    }

    #[test]
    fn test_print_all() {
        let mut list = DoubleLinkedList::create();
        list.add_value(&100);
        list.add_value(&200);
        list.add_value(&300);
        // This just checks that print_all runs without panicking
        list.print_all();
        list.print_all_backwards();
    }
}
