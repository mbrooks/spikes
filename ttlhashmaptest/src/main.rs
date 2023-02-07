use std::time::{Duration};
use std::thread;
use ttlhashmap::{TtlHashMap, AutoClean};

fn main() {
  let mut map = TtlHashMap::new(Duration::from_secs(1));
  map.autoclean = AutoClean::Never;

  // test1 will be deleted (by expiry)
  map.insert("test1", "This is test1's value");
  thread::sleep(Duration::from_secs(2));
  // test2 will be deleted (by maximum nodes bound)
  map.insert("test2", "This is test2's value");
  thread::sleep(Duration::from_millis(50));
  // test3 will remain
  map.insert("test3", "This is test3's value");

  assert_eq!(map.len(), 3);

  // Set the maximum number of nodes to 1
  map.max_nodes = Some(1);

  // Explicitly pass an maximum number of nodes
  map.cleanup();

  assert_eq!(map.len(), 1);

  assert_eq!(map.contains_key("test1"), false);
  assert_eq!(map.contains_key("test2"), false);
  assert_eq!(map.contains_key("test3"), true);
}