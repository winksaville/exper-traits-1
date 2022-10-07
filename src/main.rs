// Trait for States
pub trait State<'a, SM, P> {
    fn process(&self, sm: &'a mut SM, msg: &P);
}

// Create a Protocal with two messages
struct State1;

impl<'a> State<'a, Sm1<'a>, Protocol1> for State1 {
    fn process(&self, sm: &'a mut Sm1, msg: &Protocol1) {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                sm.data1 += f1;
                println!("State1: process sm.data1={} Msg1::f1={}", sm.data1, f1);
            }
        }
    }
}

#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
}

struct Sm1<'a> {
    current_state: &'a dyn State<'a, Self, Protocol1>,
    data1: i32,
}

impl<'a> Sm1<'a> {
    fn new(s: &'a dyn State<'a, Self, Protocol1>) -> Self {
        Sm1 {
            current_state: s,
            data1: 1,
        }
    }
}

fn main() {
    // Create State1
    let s1 = State1;

    // Create State Machine and set State1 as current state
    let mut sm = Sm1::new(&s1);

    // Create a message
    let msg = Protocol1::Msg1 { f1: 123 };

    // Processes a message
    sm.current_state.process(&mut sm, &msg);
}
