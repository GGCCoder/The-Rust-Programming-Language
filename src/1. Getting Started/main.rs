// error[E0423]: expected function, found macro `println`
//  --> main.rs:2:3
//   |
// 2 |   println("Hello, World!");
//   |   ^^^^^^^ help: use `!` to invoke the macro: `println!`

// error: aborting due to previous error

fn main() {
  println!("Hello, World!");
}