pub struct Cart {
   name: String
}

impl Cart {
  pub fn new() -> Cart {
      Cart{ name: String::from_str("Hello") }
  }
}
