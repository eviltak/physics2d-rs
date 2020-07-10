struct PoolObject<T: Default> {
    data: T,
    next: PoolId,
}

impl<T: Default> PoolObject<T> {
    pub fn new(next: PoolId) -> PoolObject<T> {
        PoolObject {
            data: T::default(),
            next,
        }
    }
}

pub type PoolId = usize;

pub struct Pool<T: Default> {
    object_pool: Vec<PoolObject<T>>,
    pub object_count: usize,
    next_free_object_id: PoolId,
}

impl<T: Default> Pool<T> {
    pub fn new(initial_capacity: PoolId) -> Pool<T> {
        let node_pool = Vec::with_capacity(initial_capacity);
        
        Pool {
            object_pool: node_pool,
            object_count: 0,
            next_free_object_id: 0,
        }
    }
    
    pub fn allocate_with(&mut self, data: T) -> PoolId {
        assert!(self.next_free_object_id <= self.object_pool.len());
    
        if self.next_free_object_id == self.object_pool.len() {
            // Expand our node pool
            self.object_pool.push(PoolObject::new(self.next_free_object_id + 1));
        }
    
        self.object_count += 1;
    
        let object_id = self.next_free_object_id;
        
        self.object_pool[object_id].data = data;
        self.next_free_object_id = self.object_pool[object_id].next;
        
        object_id
    }
    
    pub fn allocate(&mut self) -> PoolId {
        self.allocate_with(T::default())
    }
    
    pub fn free(&mut self, id: PoolId) {
        let pool_object = self.object_pool.get_mut(id).expect("Invalid object id");
        *pool_object = PoolObject::new(self.next_free_object_id);
        self.next_free_object_id = id;
        
        self.object_count -= 1;
    }
    
    pub fn get(&self, id: PoolId) -> &T {
        &self.object_pool.get(id).expect("Invalid object id").data
    }
    
    pub fn get_mut(&mut self, id: PoolId) -> &mut T {
        &mut self.object_pool.get_mut(id).expect("Invalid object id").data
    }
}

impl<T: Default> Default for Pool<T> {
    fn default() -> Pool<T> {
        Pool::new(64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn allocate() {
        let mut pool = Pool::default();
        let id_a = pool.allocate();
        {
            let a = pool.get_mut(id_a);
            *a = 32u32;
        }
        assert_eq!(pool.object_count, 1);
        assert_eq!(pool.next_free_object_id, id_a + 1);
        assert_eq!(pool.get(id_a), &32);
    }
    
    #[test]
    fn free() {
        let mut pool = Pool::default();
        let (_a, b, _c) = (pool.allocate(), pool.allocate(), pool.allocate());
        
        {
            *pool.get_mut(b) = 111u32;
        }
        
        pool.free(b);
        
        assert_eq!(pool.object_count, 2);
        assert_eq!(pool.next_free_object_id, b);
        
        let new_id = pool.allocate();
        
        assert_eq!(b, new_id);
        assert_eq!(pool.get(new_id), &u32::default())
    }
}
