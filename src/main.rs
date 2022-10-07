// Trait for States
pub trait State<'a, SM, P> {
    fn process(&self, sm: &'a mut SM, msg: &P);
}

// Create a Protocal with two messages
struct State1;

impl<'a> State<'a, MySm<'a>, Protocol1> for State1 {
    fn process(&self, sm: &'a mut MySm, msg: &Protocol1) {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                sm.data1 += 1;
                println!("State1: process sm.data1={} Msg1::f1={}", sm.data1, f1);
            }
        }
    }
}

#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
}

struct MySm<'a> {
    current_state: &'a dyn State<'a, Self, Protocol1>,
    data1: i32,
}

impl<'a> MySm<'a> {
    fn new(s: &'a dyn State<'a, Self, Protocol1>) -> Self {
        MySm {
            current_state: s,
            data1: 1,
        }
    }
}

fn main() {
    // Create State1
    let s1 = State1;

    // Create State Machine and State1 as current state
    let mut mysm = MySm::new(&s1);

    // Allocate a message on the stack and dispatch it
    let msg = Protocol1::Msg1 { f1: 123 };

    // Processes a message
    mysm.current_state.process(&mut mysm, &msg)
}
