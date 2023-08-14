// Definição da estrutura do nó da lista encadeada LIFO
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            next: None,
        }
    }
}

// Definição da estrutura da lista encadeada
struct Stack<T> {
    head: Option<Box<Node<T>>>,
}


impl<T: std::fmt::Debug + std::cmp::PartialEq + std::fmt::Display> Stack<T> {
    fn new() -> Stack<T> {
        Stack { head: None }
    }

    fn push_front(&mut self, data: T) {
        if self.repetido(&data) {
            println!("\n!!!! repetido: {} !!!!\n", data);
        } else {
            let mut new_node = Box::new(Node::new(data));
            new_node.next = self.head.take();
            self.head = Some(new_node);
        }
    }

    // Remove o primeiro elemento da lista
    fn pop(&mut self) {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
        });
    }

    // Remove o último elemento da pilha
    fn pop_back(&mut self) {
        if self.head.is_none() {
            return;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            self.pop();
        } else {
            let mut current = &mut self.head;
            while current.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }
            current.as_mut().unwrap().next = None;
        }
    }

    // Adiciona um elemento no final da pilha
     fn push_back(&mut self, value: T) {
        if self.repetido(&value) {
            println!("\n!!!! repetido: {} !!!!\n", value);
        } else {
            if self.head.is_none() {
                self.push_front(value);
                return;
            }

            let mut current = &mut self.head;
            while current.as_ref().unwrap().next.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }

            let new_node = Box::new(Node::new(value));
            current.as_mut().unwrap().next = Some(new_node);
        }
    }

   // Retorna o primeiro elemento da lista
    fn get_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // Retorna o último elemento da pilha
    fn get_back(&self) -> Option<&T> {
        if self.head.is_none() {
            return None;
        }

        let mut current = &self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.data);
            }
            current = &node.next;
        }
        None
    }

    // Imprime os elementos da lista
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.data);
            current = &node.next;
        }
    }
    
    // confere se um elemento esta na lista
    fn repetido(&self, value: &T) -> bool {
    	let mut flag: bool = false;
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == *value{
                flag = true;
            }
            current = &node.next;
        }
        flag
    }
}

// Função principal
fn main() {
    let mut my_stack: Stack<i32> = Stack::new();

    for _i in 1..=5 { my_stack.push_front(_i); }
    my_stack.push_front(2);
    for _i in (1..=5).rev() { my_stack.push_back(_i); }

    println!("Lista:");
    my_stack.print();
    let front : i32 = *my_stack.get_front().unwrap();
    let back  : i32 = *my_stack.get_back().unwrap();
    if Some(front).is_some() && Some(back).is_some() {
       println!("Primeiro: {:?}, último: {:?}",front,back);
    }

    println!("-+-+-+ Tirando o primeiro elemento -+-+-+");
    my_stack.pop();
    println!("Lista:");
    my_stack.print();
    let front : i32 = *my_stack.get_front().unwrap();
    let back  : i32 = *my_stack.get_back().unwrap();
    if Some(front).is_some() && Some(back).is_some() {
       println!("Primeiro: {:?}, último: {:?}",front,back);
    }

    println!("-+-+-+ Tirando o último elemento -+-+-+");
    my_stack.pop_back();
    println!("Lista:");
    my_stack.print();
    let front : i32 = *my_stack.get_front().unwrap();
    let back  : i32 = *my_stack.get_back().unwrap();
    if Some(front).is_some() && Some(back).is_some() {
       println!("Primeiro: {:?}, último: {:?}",front,back);
    }

    println!("-+-+-+ Inserindo no início -+-+-+");
    my_stack.push_front(100);
    println!("Lista:");
    my_stack.print();
    let front : i32 = *my_stack.get_front().unwrap();
    let back  : i32 = *my_stack.get_back().unwrap();
    if Some(front).is_some() && Some(back).is_some() {
       println!("Primeiro: {:?}, último: {:?}",front,back);
    }

    println!("-+-+-+ Inserindo no final -+-+-+");
    my_stack.push_back(999);
    my_stack.push_back(999);
    println!("Lista:");
    my_stack.print();
    let front : i32 = *my_stack.get_front().unwrap();
    let back  : i32 = *my_stack.get_back().unwrap();
    if Some(front).is_some() && Some(back).is_some() {
       println!("Primeiro: {:?}, último: {:?}",front,back);
    }
}
