// OWNERSHIP and BORROWING RULES

struct Bar {
    x: i32,
}

struct FooBar {
    x: Bar,
}

struct Foo {
    x: i32,
}

pub fn run() {
    /* OWNERSHIP */
    // Instantiating a type and binding it to a variable name creates a memory resource that the
    // Rust compiler will validate through its whole lifetime. The bound variable is called the
    // resource's owner.

    // We instantiate structs and bind to variables
    // to create memory resources
    let foo = Foo { x: 42 };
    // foo is the owner
    // FOO IS THE FATHER :P

    /* Scope-Based Resource Management */

    // When a struct is dropped, the struct itself is dropped first, then its children are dropped
    // individually, and so on.

    let foo = FooBar { x: Bar { x: 42 } };
    println!("{}", foo.x.x);

    // foo is dropped first
    // then foo.bar is dropped

    /* Dropping is Hierarchical */

    //  When an owner is passed as an argument to a function, ownership is moved to the function
    //  parameter.
    //
    //  After a move the variable in the original function can no longer be used.
    //
    //  Memory details:
    //
    //      During a move the stack memory of the owners value is copied to the function call's
    //      parameter stack memory.
    //

    let foo = Foo { x: 42 };
    // foo is moved to do_something
    do_something(foo);
    // foo can no longer be used

    /* Returning Ownership */
    // Ownership can also be returned from a function.
    let foo = return_ownership();
    // foo becomes the owner
    // foo is dropped because of end of function scope

    /* Borrowing Ownership with References */

    //References allow us borrow access to a resource with the & operator.
    //
    //References are also dropped like other resources.

    let foo = Foo { x: 42 };
    // now REFERENCING foo in f
    let f = &foo;
    println!("{}", f.x);
    // f dropped first because f is a reference and the reference needs to be
    // dropped first to not cause bugs
    // f is dropped here
    // foo is dropped here

    /* Borrowing Mutable Ownership with References */

    // We can also borrow mutable access to a resource with the &mut operator.
    // A resource owner cannot be moved or modified while mutably borrowed.
    //
    //    Rust prevents having two ways to mutate an owned value because it introduces the
    //    possibility of a data race.
    //

    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // FAILURE: do_something(foo) would fail because
    // foo cannot be moved while mutably borrowed
    //
    // FAILURE: foo.x = 13; would fail here because
    // foo is not modifiable while mutably borrowed

    f.x = 13;
    // f is dropped here because it's no longer used after this point
    println!("{}", foo.x);
    // this works now because all mutable references were dropped
    foo.x = 7;
    // move foo's ownership to a function
    do_something(foo);
    /* Dereferencing */
    //Using &mut references, you can set the owner's value using the * operator.
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // get a copy of the owner's value
    *f = 13; // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);

    /* Passing Around Borrowed Data */
    let mut foo = Foo { x: 42 };
    add_one(&mut foo);
    println!("foo.x = {}", foo.x);
    // because all mutable references are dropped within
    // the function do_something, we can create another.
    add_one(&mut foo);
    println!("foo.x = {}", foo.x);
    // foo is dropped here
    
    /* References Of References */
    // References can even be used on pieces of references.
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    // now foo.x is 13
    *x = 13;
    // x is dropped here allow us to create a non-mutable reference
   let y = return_reference(&foo);
   // println!("{}", y);
    // y is dropped here
    // foo is dropped here

    // foo is dropped here
    // which means goodbye foo
} // Rust uses the end of scope as the place to deconstruct and deallocate a resource.
  // The term for this deconstruction and deallocation is called a drop.

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f (which was foo before passed as a argument) is dropped here
}
fn return_ownership() -> Foo {
    // return
    Foo { x: 42 }
    // ownership is moved out
}

fn return_reference(a: &Foo) -> &i32 {
    return &a.x;
}
fn add_one(f: &mut Foo) {
    f.x += 1;
    // mutable reference f is dropped here
}

