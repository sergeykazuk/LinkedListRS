

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use single_linked_list::SingleLinkedList;
    
    #[test]
    fn test_add_and_at() {
        let mut list = SingleLinkedList::new();
        list.push_back(&1);
        list.push_back(&2);
        list.push_back(&3);
        assert_eq!(list.size(), 3);
        assert_eq!(list.at(0), Some(1));
        assert_eq!(list.at(1), Some(2));
        assert_eq!(list.at(2), Some(3));
        assert_eq!(list.at(3), None);
    }

    #[test]
    fn test_pop() {
        let mut list = SingleLinkedList::new();
        list.push_back(&10);
        list.push_back(&20);
        list.push_back(&30);
        list.push_back(&40);
        assert_eq!(list.size(), 4);

        let front = list.pop_front();
        assert_eq!(front, Some(10));
        assert_eq!(list.size(), 3);

        let front = list.pop_back();
        assert_eq!(front, Some(40));
        assert_eq!(list.size(), 2);

        assert_eq!(list.at(0), Some(20));
        assert_eq!(list.at(1), Some(30));
        assert_eq!(list.at(2), None);
    }

    #[test]
    fn test_remove() {
        let mut list = SingleLinkedList::new();
        list.push_back(&10);
        list.push_back(&20);
        list.push_back(&30);
        list.push_back(&40);
        assert_eq!(list.size(), 4);

        let front = list.remove_value(&30);
        assert_eq!(list.size(), 3);

        list.remove_value(&50);
        assert_eq!(list.size(), 3);
    }


}
