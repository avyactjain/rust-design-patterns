use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Command trait
trait Command {
    fn execute(&self);
    fn undo(&self);
}

// Receiver - Light device
struct Light {
    is_on: bool,
}

impl Light {
    fn new() -> Self {
        Self { is_on: false }
    }

    fn turn_on(&mut self) {
        self.is_on = true;
        println!("Light is ON");
    }

    fn turn_off(&mut self) {
        self.is_on = false;
        println!("Light is OFF");
    }
}

// Concrete Commands
struct TurnOnLightCommand {
    //Rc allows multiple owners of T
    //RefCell<T> is a smart pointer in Rust that allows interior mutability
    //Rc<Refcell<T>> is very powerful as it will allow T to have multiple owners which can all mutate it safely.
    light: Rc<RefCell<Light>>,
}

impl Command for TurnOnLightCommand {
    fn execute(&self) {
        self.light.borrow_mut().turn_on();
    }

    fn undo(&self) {
        self.light.borrow_mut().turn_off();
    }
}

struct TurnOffLightCommand {
    light: Rc<RefCell<Light>>,
}

impl Command for TurnOffLightCommand {
    fn execute(&self) {
        self.light.borrow_mut().turn_off();
    }

    fn undo(&self) {
        self.light.borrow_mut().turn_on();
    }
}

// Invoker - Remote Control
struct RemoteControl {
    history: VecDeque<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        Self {
            history: VecDeque::new(),
        }
    }

    fn press_button(&mut self, command: Box<dyn Command>) {
        command.execute();
        self.history.push_back(command);
    }

    fn press_undo(&mut self) {
        if let Some(last_command) = self.history.pop_back() {
            last_command.undo();
        } else {
            println!("No commands to undo.");
        }
    }
}

// Main function
fn main() {
    let light = Rc::new(RefCell::new(Light::new()));
    let mut remote = RemoteControl::new();

    let turn_on = Box::new(TurnOnLightCommand {
        light: Rc::clone(&light),
    });
    let turn_off = Box::new(TurnOffLightCommand {
        light: Rc::clone(&light),
    });

    remote.press_button(turn_on); // Light ON
    remote.press_button(turn_off); // Light OFF

    remote.press_undo(); // Undo last (Light ON)
    remote.press_undo(); // Undo again (Light OFF)
}
