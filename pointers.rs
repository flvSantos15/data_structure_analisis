fn main() {
  ex()
}

fn ex() {
  let mut x: i32 = 5;

  let y = &mut x;

  *y = 10;

  println("x: {}", y)
}