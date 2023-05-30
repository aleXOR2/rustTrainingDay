 // https://rust-book.cs.brown.edu/ch04-04-slices.html



fn main() {
  let s = String::from("hello");
  let s2: &String = &s;
  let s3: &str = &s[..];
  println!(
    "Void={} &String={} &str={}",
    std::mem::size_of::<()>(),
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
  );
}
