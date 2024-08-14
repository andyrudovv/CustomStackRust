use linked_list::LinkedList;

mod linked_list;

fn main()  {
    let mut l = LinkedList::<i32>::new();
    l.add(1);
    l.add(2);
    l.add(3);
    l.add(4);
    l.add(5);

    let mut i = l.clone().into_iter();
    for _ in 0..l.get_len() {
        println!("{}", i.next().unwrap());
    }

    let last = l.peek_mut();
    

    
    println!("{:?}", last);

    let mut i = l.clone().into_iter();
    for _ in 0..l.get_len() {
        println!("{}", i.next().unwrap());
    }
    println!("Done!");
}