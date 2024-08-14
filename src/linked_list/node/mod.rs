

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>
}