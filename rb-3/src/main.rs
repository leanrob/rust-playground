fn main() {

    // This function takes an immutable reference
    let print_vector = |x:&Vec<i32>| {
        println!("{:?}", x)
    };

    // Borrowing
    let v = vec![3,2,1];

    // & lets this function borrow this vector for a while
    print_vector(&v);

    // Now we can legally print this after it is used, because it was borrowed
    println!("v[0] = {}", v[0]);

    ////

    // Note: There can only be 1 mutable reference to a resource
    // - but there can be as many non-mutable references as we want
    let mut a = 40;

    // A new scope, so that when the scope ends b "release" ownership of a
    {
        // make a reference to a
        let b = &mut a;

        // * means the we want to use the thing that the reference references
        *b += 2;
    }

    println!("a = {}", a);
}
