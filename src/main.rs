use std::{cell::OnceCell, rc::Rc};

trait Node {
    type Output;
    fn execute(&self) -> Self::Output;
}

struct Adder<A, B> 
where
    A: Node<Output = i32>,
    B: Node<Output = i32>,
{
    a: Rc<A>,
    b: Rc<B>,
    result: OnceCell<i32>,
}

impl<A: Node<Output = i32>, B: Node<Output = i32>> Adder<A, B> {
    fn new(a: Rc<A>, b: Rc<B>) -> Self {
        Self { a, b, result: OnceCell::new() }
    }
}

impl<A, B> Node for Adder<A, B>
where
    A: Node<Output = i32>,
    B: Node<Output = i32>,
{
    type Output = i32;
    fn execute(&self) -> i32 {
        println!("Computing something ;)");
        *self.result.get_or_init(|| self.a.execute()+self.b.execute())
    }
}

struct Const(i32);
impl Node for Const {
    type Output = i32;
    fn execute(&self) -> i32 { self.0 }
}

struct Program<N> {
    last_node: N,
}

impl<N: Node> Program<N> {
    fn run(&self) -> N::Output { self.last_node.execute() }
}

fn main() {
    let expensive_computation = Rc::new(Adder::new(
        Rc::new(Const(50)),
        Rc::new(Const(123123))
    ));
    let add = Adder::new(expensive_computation.clone(), expensive_computation.clone());
    let program = Program { last_node: add };
    println!("{}", program.run());
}
