use xorlist::XorList;

fn main() {
    let mut list = XorList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.push_back(4);
    list.insert(5, 2);
    list.traverse(|val| print!("{} ", val));
    println!("\nInsert Traversion");
    list.delete(2);
    list.traverse(|val| print!("{} ", val));
    println!("\nTraversion after delete");
}
