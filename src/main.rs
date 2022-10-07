// Trait for States
pub trait State<'a, P> {
    fn process(&self, msg: &P);
}

// Create a Protocal with two messages
struct State1;

impl<'a> State<'a, Protocol1> for State1 {
    fn process(&self, msg: &Protocol1) {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                println!("State1: process Msg1::f1={}", f1);
            }
        }
    }
}

#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
}

struct Sm1<'a> {
    states: Vec<&'a dyn State<'a, Protocol1>>,
    current_state_idx: usize,
    //current_state: &'a dyn State<'a, Self, Protocol1>,
    data1: i32,
}

impl<'a> Sm1<'a> {
    fn new(s: &'a dyn State<'a, Protocol1>) -> Self {
        Sm1 {
            states: vec![s],
            current_state_idx: 0,
            //current_state: s,
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
    sm.states[sm.current_state_idx].process(&msg);

    sm.data1 += match msg {
        Protocol1::Msg1 { f1 } => f1,
    };
    println!("sm.data1: {}", sm.data1);
}
