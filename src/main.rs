// Trait for States
pub trait State<'a, SM, P> {
    fn process(&self, sm: &'a /*mut*/ SM, msg: &P); // -> &'a mut SM;
}

// Create a Protocal with two messages
struct State1;

impl<'a> State<'a, MySm<'a>, Protocol1> for State1 {
    fn process(&self, sm: &'a /*mut*/ MySm<'a>, msg: &Protocol1) { // -> &'a mut MySm<'a> {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                //sm.data1 += f1;
                println!("State1: process sm.data1={:3}      added Msg1::f1={}", sm.data1, f1);
            }
        }

        //sm
    }
}

struct State2;

impl<'a> State<'a, MySm<'a>, Protocol1> for State2 {
    fn process(&self, sm: &'a /*mut*/ MySm<'a>, msg: &Protocol1) { // -> &'a mut MySm<'a> {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                //sm.data1 -= f1;
                println!("State2: process sm.data1={:3} subtracted Msg1::f1={}", sm.data1, f1);
            }
        }

        //sm
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
            data1: 0,
        }
    }

    fn dispatch(&'a /*mut*/ self, msg: &Protocol1) { // -> &'a mut MySm<'a> {
        self.current_state.process(self, msg);
    }
}

fn main() {
    // Create State1
    let s1 = State1;
    let s2 = State2;

    // Create State Machine and State1 as current state
    let mut sm = MySm::new(&s1);
    println!("Create sm       sm.data1={}", sm.data1);

    // Allocate a message on the stack and dispatch it
    let msg = Protocol1::Msg1 { f1: 123 };

    // Processes a message
    //let mut sm = sm.dispatch(&msg);
    sm.dispatch(&msg);

    sm.current_state = &s2;
    sm.dispatch(&msg);
}
