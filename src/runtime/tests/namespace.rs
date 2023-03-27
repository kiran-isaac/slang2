use crate::runtime::namespace::Namespace;

#[test]
pub fn namespace_create() {
  let n1 = Box::new(Namespace::new("test".to_string(), Option::None));
  assert_eq!(n1.name, "test".to_string());

  let n2 = Namespace::new("test2".to_string(), Option::Some(n1));
  assert_eq!(n2.full_name, "test.test2".to_string());
}