use core::option::Option;

#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    buckets: [Option<(K, V)>; 10],  
}

impl<K: Eq, V> HashMap<K, V> {
    // 创建一个新的 HashMap
    pub fn new() -> Self {
        HashMap {
            buckets: [None, None, None, None, None, None, None, None, None, None], 
        }
    }

    // 插入一个键值对
    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_index(&key);
        self.buckets[index] = Some((key, value));
    }

    // 查询键对应的值
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_index(key);
        match &self.buckets[index] {
            Some((ref stored_key, ref value)) if stored_key == key => Some(value),
            _ => None,
        }
    }

    // 根据键返回对应的数组索引
    fn get_index(&self, key: &K) -> usize {
        let key_ptr = core::ptr::addr_of!(key); 
        (key_ptr as usize) % self.buckets.len()  
    }

   // 迭代器
    pub fn iter(&self) -> HashMapIter<K, V> {
        HashMapIter {
            map: self,
            index: 0,
        }
    }
}

// 迭代器结构体
pub struct HashMapIter<'a, K, V> {
    map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K: Eq, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.buckets.len() {
            if let Some((ref key, ref value)) = self.map.buckets[self.index] {
                self.index += 1;
                return Some((key, value));
            }
            self.index += 1;
        }
        None
    }
}
