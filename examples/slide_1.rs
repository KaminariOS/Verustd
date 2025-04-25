fn main() {
    // s represent ownership of a memory object
    // Here, s is a string on the heap
    let mut s = String::from("hello"); // s owns the String
    // the ownership can be borrowed temporarily 
    borrow_string_mut(&mut s);
    // s is moved into sink, no longer in current scope
    sink(s)
    // println!("{}", s);          // ERROR: use of moved value “s”
}

// Immutable borrow 
fn borrow_string(s: &String) {
    // ERROR: cannot mutate 
    // s.pop();
}

// Mutable borrow 
fn borrow_string_mut(s: &mut String) {
    s.pop();
}

fn sink(s: String) {
    // s goes out of scope and its destructor gets called automatically 
}
