trait Node {
    type Output;
    fn execute(&self) -> Self::Output;
}

struct Adder<A, B> 
where
    A: Node<Output = i32>,
    B: Node<Output = i32>,
{
    a: A,
    b: B,
}
impl<A, B> Node for Adder<A, B>
where
    A: Node<Output = i32>,
    B: Node<Output = i32>,
{
    type Output = i32;
    fn execute(&self) -> i32 {
        return self.a.execute()+self.b.execute()
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
    let add = Adder {
        a: Const(1),
        b: Adder { 
            a: Const(1), 
            b: Const(2) 
        },
    };
    let program = Program { last_node: add };
    println("{}", program.run());
}
