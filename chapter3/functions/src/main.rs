fn main() {
    let y = roundabout_four();
    println!("That was a weird way of printing {y}");
}

fn roundabout_four() -> i32 {
    // statement: not an evaluation, doesnt return anything
    // expression: returns something.
    // when we define a variable, we need an expression.
    // so let y = (x = 4) is invalid, because there isn't a return type to latch onto.
    let y = {
        let x = 3; // statement
        x + 1 // expression
    }; // statement
    // because we have an expression, this is valid.
    return y;
}
