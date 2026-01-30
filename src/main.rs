
fn main() {
    // outer scope
    let x:i32 = 10;
    let y:i32 = 20;

    // inner scope
    {
	let x:i32 = 100;
	let y:i32 = 200;
	println!("result for x: {}", x);
	println!("result for y: {}", y);	      
    }
    // outer scope
    println!("result for x: {}", x);
    println!("result for y: {}", y);	      
}
