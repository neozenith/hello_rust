fn main() {
  add(5,7);
}
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
  
  use super::*;

  #[test]
  fn test_add(){
    assert_eq!(add(1,2), 3)
  }
  
  #[test]
  fn test_many_add(){
    for i in 1..1000000000 {
      assert_eq!(add(i,i), 2*i)
    }
  }
    
}
