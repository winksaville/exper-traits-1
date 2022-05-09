// Trait for Finite State Machines
trait StateMachine<P> {
    /// Return the current state
    fn cur_state(&mut self) -> &mut Box<dyn State<Self, P> + 'static>;

    fn set_state(&mut self, next_state: Box<dyn State<Self, P> + 'static>);

    fn dispatch(&mut self, msg: &P)
    where
        Self: Sized,
    {
        self.cur_state().process(self, msg);
    }
}

// Trait for States
pub trait State<SM, P> {
    fn process(&mut self, _sm: &mut SM, _msg: &P) {
        println!("State: process");
    }
}

// Create a Protocal with two messages
#[derive(Debug)]
enum Protocol1 {
    Msg1 { f1: i32 },
}

struct State1;

impl State<MySm<Protocol1>, Protocol1> for State1 {
    fn process(&mut self, sm: &mut MySm<Protocol1>, msg: &Protocol1) {
        match *msg {
            Protocol1::Msg1 { f1 } => {
                sm.data1 += 1 as f32;
                println!("State1: process sm.data1={} Msg1::f1={}", sm.data1, f1);
            }
        }
    }
}

struct MySm<Protocol1> {
    current_state: Box<dyn State<Self, Protocol1> + 'static>,
    data1: f32,
}

impl<Protocol1> MySm<Protocol1> {
    fn new(s: Box<dyn State<Self, Protocol1> + 'static>) -> Self {
        MySm {
            current_state: s,
            data1: 1.0,
        }
    }
}

impl<Protocol1> StateMachine<Protocol1> for MySm<Protocol1> {
    fn cur_state(&mut self) -> &mut Box<dyn State<Self, Protocol1> + 'static> {
        println!("StateMachine::cur_state():+-");
        &mut self.current_state
    }

    fn set_state(&mut self, next_state: Box<dyn State<Self, Protocol1> + 'static>) {
        println!("StateMachine::set_state():+-");
        self.current_state = next_state;
    }
}

fn main() {
    // Create state machine and initialize to State1
    let mut mysm = Box::new(MySm::new(Box::new(State1 {})));

    // Allocate a message on the stack and dispatch it
    let msg = Protocol1::Msg1 { f1: 123 };
    mysm.dispatch(&msg);
}
