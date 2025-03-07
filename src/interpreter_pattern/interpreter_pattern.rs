trait Expression {
    fn interpret(&self) -> i32;
}

struct Number {
    value: i32,
}

impl Expression for Number {
    fn interpret(&self) -> i32 {
        self.value
    }
}

struct Multiply {
    left_val: Box<dyn Expression>,
    right_val: Box<dyn Expression>,
}
impl Expression for Multiply {
    fn interpret(&self) -> i32 {
        self.left_val.interpret() * self.right_val.interpret()
    }
}

struct Add {
    left_val: Box<dyn Expression>,
    right_val: Box<dyn Expression>,
}
impl Expression for Add {
    fn interpret(&self) -> i32 {
        self.left_val.interpret() + self.right_val.interpret()
    }
}

struct Subtract {
    left_val: Box<dyn Expression>,
    right_val: Box<dyn Expression>,
}
impl Expression for Subtract {
    fn interpret(&self) -> i32 {
        self.left_val.interpret() - self.right_val.interpret()
    }
}

fn main() {
    let three = Number { value: 3 };
    let two = Number { value: 2 };
    let five = Number { value: 5 };
    let seven = Number {value : 7};

    let two_times_five = Multiply {
        left_val: Box::new(two),
        right_val: Box::new(five),
    };

    let sevel_plus_two_times_five_minus_3_times_2 = Add { left_val: Box::new(seven), right_val: Box::new(Subtract {
        left_val: Box::new(two_times_five),
        right_val: Box::new(Multiply {
            left_val: Box::new(three),
            right_val: Box::new(Number { value: 2 }),
        }),
    }) };

    println!(
        "7 + (2*5) - (3*2) = {}",
        sevel_plus_two_times_five_minus_3_times_2.interpret()
    );
}
