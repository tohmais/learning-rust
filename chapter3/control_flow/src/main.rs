fn main() {
    // rust makes you pass in a BOOL value into an if statement.
    // so if i said let a = 0, this does not compile.
    let a = 1 == 1; // should always be true.
    if a {
        println!("Everything's ok.");
    } else {
        println!("Something's gone horribly, horribly wrong.");
    }

    // so instead of querying "if b", you have to say...
    let b = 20;
    if b != 0 {
        println!("See? This works.");
    } else {
        print!("You might want to get your computer checked... ");
        println!("or go to your nearest bomb shelter.");
    }

    // This should print FooBar.
    let mut a = 35;
    if a % 5 == 0{
        print!("Foo");
    }
    if a % 7 == 0 {
        println!("Bar");
    }

    // however, this will only print 'Foo'.
    // this is because else statements are only evaluated
    // if the initial condition is false.

    if a % 5 == 0{
        println!("Foo");
    } else if a % 7 == 0{
        println!("Bar");
    }


    a = 0;
    loop {
        if a == 0 {
            println!("A loop is a structure that's useful in many scenarios.");
            println!("Let's say you wanted to count to 10.");
        } else if a == 10 {
            println!("{}.", a);
            break;
        } else {
            print!("{}, ", a);
        }
        a += 1;
    }
    
    println!("Not bad, right?");
    println!("But there's some better ways to do this.");

    a = 1;
    while a <= 10 {
        if a == 10 {
            println!("{}.", a);
        } else {
            print!("{}, ", a);
        }
        a += 1;
    }

    println!("This one is even cooler, though.");

    for n in 1..11 {
        if n == 10 {
            println!("{}.", n);
        } else {
            print!("{}, ", n);
        }
    }
}
