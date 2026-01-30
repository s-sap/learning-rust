fn main() {
    let x :i32 = 5; // x is immutable
    let x = x + 1; //  new x is now shadowing the old x variable
    println!("x is now: {}", x); // 6
    {
	let x = x * 2;  // inner scope x 
	println!("Inner Scope x is now: {}", x); // 12

    }

    println!("the value of outer scope x: {}", x); // 6
}
