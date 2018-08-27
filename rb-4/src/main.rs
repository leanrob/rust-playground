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

fn print_vector(v:&Vec<i32>) -> i32 {
    return v[0];
}

// Expect this test ot pass
#[test]
fn vector_first_value_correct() {
    let v = vec![3,2,1];
    assert_eq!(3, print_vector(&v));
}

// Expect this test to panic and fail
#[test]
#[should_panic]
fn vector_first_value_panic() {
    let v = vec![3,2,1];
    assert_eq!(42, print_vector(&v));
}

// This test is ignored and shows as such in the console
#[test]
#[should_panic]
#[ignore]
fn vector_first_value_ignored() {
    let v = vec![3,2,1];
    assert_eq!(42, print_vector(&v));
}
