use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

// Example 1:------------------------------------------------------------------
// Memory Peril
struct Dog {
    pub name: String,
}

impl Dog {
    #[allow(dead_code)]
    fn print_name(&self) {
        println!("I am {}", self.name);
    }
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("End of {}", self.name);
    }
}
/*
//Does not compile! The compiler requires a lifetime param for the reference.
fn make_bad_dog() -> &Dog {
    let spike = Dog { name: String::from("Spike") };

    let mut d: &mut Dog = &mut spike;
    {
        let snoopy = Dog { name: String::from("Snoopy" )};
        d = &snoopy;
    }

    d
}

// Does not compile! See below ↓
fn make_bad_dog() -> *mut Dog {
    let mut spike = Dog { name: String::from("Spike") };

    let mut d: &mut Dog = &mut spike;
    {
        let mut snoopy = Dog { name: String::from("Snoopy" )};
        d = &mut snoopy; // The compiler knows snoopy is about to go out of
                         // scope, so it will complain about this.
    }

    d
}
*/
// Hmm, okay, the compiler is okay with this function...
#[allow(dead_code)]
fn make_bad_dog() -> *mut Dog {
    let mut spike = Dog { name: String::from("Spike") };
    let d: &mut Dog = &mut spike;
    d
}

/*
// But, boom! We fail here ↓
fn bad_stack() {
    let d = make_bad_dog();
    // We've tricked the compiler up to this point, but now it knows no
    // good will come of this raw pointer deref, so it complains until
    // we wrap this in an unsafe{} block, to prove to it we accept the
    // consequences.
    (*d).print_name();
}
*/

// ----------------------------------------------------------------------------

// Example 2:------------------------------------------------------------------
// Verbosity of Mutability 1

fn do_with_str(string: String) {
    let mut strs = Vec::<String>::new();
    strs.push(string);
    println!("{}", strs[0]);
}

fn pass_vec() {
    let string = String::from("Test");
    do_with_str(string);

    // This line fails compilation, because we've use string after moving it
    // to another owner.
    // println!("{}", string);
}

// ----------------------------------------------------------------------------

// Example 3:------------------------------------------------------------------
// Verbosity of Mutability 2

fn just_print_i_swear(string: &mut String) {
    println!("{}", string);

    *string = String::from("I lied! But what did you expect? You gave me an explicit mutable ref.");
}

fn be_deceived() {
    let mut string = String::from("Innocent String");
    just_print_i_swear(&mut string);
    println!("{}", string);
}

// Note how, in Rust, fewer tokens are required to borrow a const reference.
// In C++, forgetting access modifiers leaves things unsafe. Forgetting them
// in Rust just means you need to add mutability when you really need it.

fn just_print_i_swear_for_real(string: &String) {
    println!("{}", string);

    // This would cause a compiler error
    // *string = String::from("I lied!");
}

fn be_relieved() {
    let string = String::from("Actually Innocent String");
    just_print_i_swear_for_real(&string);
    println!("{}", string);
}

// ----------------------------------------------------------------------------

// Example 4:------------------------------------------------------------------
// Mad Threads

// Wthout safety gear, this doesn't even compile.
/*
fn mad_threads_fail() {
    for round in 0..10 {
        let mut int_str = String::from("10000");
        let mut threads = Vec::<JoinHandle<()>>::new();

        for _ in 0..100 {
            (&mut threads).push(std::thread::spawn(|| {
                for _ in 0..100 {
                    let mut value = int_str.parse::<i32>().unwrap();
                    value += 1;
                    int_str = value.to_string();
                }
            }));
        }

        for thread in threads.into_iter().rev() {
            thread.join().unwrap();
        }

        println!("({}/9) int_str: {}" , round, int_str);
    }
}
*/

// This is still pretty bad practice but hey, at least it's safe!
fn mad_threads() {
    for round in 0..10 {
        let int_str = Arc::new(Mutex::new(String::from("10000")));
        let mut threads = Vec::<JoinHandle<()>>::new();

        for _ in 0..100 {
            let int_str = int_str.clone();
            (&mut threads).push(std::thread::spawn(move || {
                for _ in 0..100 {
                    match int_str.lock() {
                        Ok(mut int_str_val) => {
                            let mut value = int_str_val.parse::<i32>().unwrap();
                            value += 1;
                            *int_str_val = value.to_string();
                        },
                        Err(err) => println!("Error locking mutex: {}", err),
                    }
                }
            }));
        }

        for thread in threads.into_iter().rev() {
            thread.join().unwrap();
        }

        println!("({}/9) int_str: {}" , round, int_str.lock().unwrap());
    }
}

// ----------------------------------------------------------------------------

// Example 5:------------------------------------------------------------------
// Don't Ignore Errors (Unless you want to)

fn something_that_may_fail() -> Result<(), ()>{
    Err(())
}

fn something_that_may_fail_with_result() -> Result<String, ()> {
    Err(())
}

/*
// This would cause compiler warnings about not handling
// the error result.
fn do_with_errors() {
    something_that_may_fail();
}
*/

#[allow(dead_code)]
// This compiles, but would crash if we ran it.
fn ignore_errors_explicitly() {
    // expect and unwrap can be used for cases where errors would be
    // unrecoverable, or so rare that they're worth just letting the app crash.
    something_that_may_fail().expect("Something failed!");
    let value = something_that_may_fail_with_result().unwrap();
    println!("{}", value);
}

// This is a very Rust-y way of handling these errors.
fn handle_failures() {
    match something_that_may_fail() {
        Err(_) => {
            println!("Something failed. Aborting...");
            return;
        },
        _ => {},
    };

    match something_that_may_fail_with_result() {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Something failed. Aborting..."),
    }
}

// ----------------------------------------------------------------------------

fn main() {
    println!("Example 1: Memory Peril straight up doesn't compile. Moving on...\n");

    println!("Example 2: Verbosity of Mutability 1");
    pass_vec();
    println!("");

    println!("Example 3: Verbosity of Mutability 2");
    be_deceived();
    be_relieved();
    println!("");

    println!("Example 4: Mad Threads");
    mad_threads();
    println!("");

    println!("5: Don't Ignore Errors (Unless you want to)");
    handle_failures();
    println!("");
}
