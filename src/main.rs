use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Hello, world!");
    let mut ht = HashTable::<String, String>::new();
    ht.put(String::from("a"), String::from("aaa"));
    ht.put(String::from("b"), String::from("bbb"));
    ht.put(String::from("aa"), String::from("aax2"));

    println!("{:?}", ht.get("a".to_string()));
    println!("{:?}", ht.get("bb".to_string()));
    println!("{:?}", ht);

    let mut ht2 = HashTable::new();
    ht2.put(1, 1);
    ht2.put(21, 21);
    ht2.put(2, 2);
    println!("{:?}", ht2);
}

#[derive(Debug)]
struct Bucket<K: Clone, V: Clone> {
    kvs: Vec<(K, V)>,
    id: i32,
}

#[derive(Debug)]
struct HashTable<K: Hash + Clone + PartialEq, V: Clone> {
    buckets: Vec<Rc<RefCell<Bucket<K, V>>>>,
}

trait Hash {
    fn get_hash(&self) -> i32;
}

impl<K: Hash + Clone + PartialEq, V: Clone> HashTable<K, V> {
    fn new() -> Self {
        HashTable {
            buckets: Vec::new()
        }
    }

    fn put(&mut self, key: K, value: V) {
        let hash = key.get_hash();
        let current_bucket;
        let bkt = self.buckets.iter().find(|b| b.borrow().id == hash);
        match bkt {
            Some(b) => {
                current_bucket = b.clone();
            },
            None => {
                let new_bucket = Rc::new(RefCell::new(Bucket {
                    id: hash,
                    kvs: Vec::new(),
                }));
                self.buckets.push(new_bucket.clone());
                current_bucket = new_bucket;
            }
        };
        current_bucket.borrow_mut().kvs.push((key, value));   
    }

    fn get(&self, key: K) -> Option<V> {
        let hash = key.get_hash();
        let bkt = self.buckets.iter().find(|b| b.borrow().id == hash);
        match bkt {
            None => None,
            Some(bucket) => {
                bucket.borrow().kvs.clone().into_iter().find(|(k,_)| *k == key).map(|(_,v)| v)
            }
        }
    }
}

impl Hash for String {
    fn get_hash(&self) -> i32 {
         self.len() as i32
    }
}

impl Hash for i32 {
    fn get_hash(&self) -> i32 {
         self % 10
    }
}
