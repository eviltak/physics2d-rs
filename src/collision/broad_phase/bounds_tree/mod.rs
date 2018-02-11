#[cfg(test)]
mod tests;

use math::{Bounds, Vec2};
use util::pool;

use std;

trait Nullable {
    const NULL: Self;
}

type NodeId = pool::PoolId;

impl Nullable for NodeId {
    const NULL: NodeId = std::usize::MAX;
}

#[derive(Clone)]
struct Node {
    bounds: Bounds,
    
    parent: NodeId,
    left: NodeId,
    right: NodeId,
    
    height: u32,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            bounds: Bounds::new(Vec2::ZERO, Vec2::ZERO),
            parent: NodeId::NULL,
            left: NodeId::NULL,
            right: NodeId::NULL,
            height: 0,
        }
    }
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.left == NodeId::NULL
    }
}

struct BoundsTree {
    pool: pool::Pool<Node>,
    pub root_id: NodeId,
}

impl BoundsTree {
    pub fn new() -> BoundsTree {
        let mut pool = pool::Pool::default();
        BoundsTree {
            root_id: NodeId::NULL,
            pool,
        }
    }
    
    pub fn get_root(&self) -> &Node {
        self.get_node(self.root_id)
    }
    
    pub fn get_root_mut(&mut self) -> &mut Node {
        let root_id = self.root_id;
        self.get_node_mut(root_id)
    }
    
    pub fn get_node(&self, node_id: NodeId) -> &Node {
        self.pool.get(node_id)
    }
    
    pub fn get_node_mut(&mut self, node_id: NodeId) -> &mut Node {
        self.pool.get_mut(node_id)
    }
    
    /// Inserts the leaf node identified by `leaf_id` into the tree. The leaf node should already 
    /// have been allocated in the node pool.
    ///
    /// When inserting the leaf node in a new branch with an existing leaf, the existing leaf is
    /// made the `left` child of the new branch, and the new leaf node is made the `right` child.
    fn insert_leaf(&mut self, leaf_id: NodeId) {
        if self.root_id == NodeId::NULL {
            self.root_id = leaf_id;
            return;
        }
    
        let leaf_bounds = self.get_node(leaf_id).bounds;
        let mut sibling_id = self.root_id;
        
        while !self.get_node(sibling_id).is_leaf() {
            let sibling = self.get_node(sibling_id);
            
            let left = self.get_node(sibling.left);
            let right = self.get_node(sibling.right);
            
            let cost_left = left.bounds.union(&leaf_bounds).perimeter() - left.bounds.perimeter();
            let cost_right = right.bounds.union(&leaf_bounds).perimeter() - right.bounds.perimeter();
            
            sibling_id = if cost_left < cost_right {
                sibling.left
            } else {
                sibling.right
            }
        }
        
        // Create new node that will become parent and replace sibling's position in the tree
        
        let parent_id = self.pool.allocate();
        
        let sibling_parent_id = self.get_node(sibling_id).parent;
        let sibling_bounds = self.get_node(sibling_id).bounds;
        let sibling_height = self.get_node(sibling_id).height;
        
        {
            let parent = self.get_node_mut(parent_id);
            parent.parent = sibling_parent_id;
            parent.bounds = leaf_bounds.union(&sibling_bounds);
            parent.height = sibling_height + 1;
            
            parent.left = sibling_id;
            parent.right = leaf_id;
        }
        
        self.get_node_mut(sibling_id).parent = parent_id;
        self.get_node_mut(leaf_id).parent = parent_id;
        
        // Place new parent in sibling's position in tree
        
        if sibling_id == self.root_id {
            self.root_id = parent_id;
        } else {
            let is_sibling_left_child = self.get_node(sibling_parent_id).left == sibling_id;
            
            if is_sibling_left_child {
                self.get_node_mut(sibling_parent_id).left = parent_id;
            } else {
                self.get_node_mut(sibling_parent_id).right = parent_id;
            }
        }
        
        // Walk back up tree balancing and fixing heights and bounds
        let mut node_id = self.get_node(leaf_id).parent;
        
        while node_id != NodeId::NULL {
            let left_id = self.get_node(node_id).left;
            let right_id = self.get_node(node_id).right;
            
            let new_height = 1 + self.get_node(left_id).height.max(self.get_node(right_id).height);
            let new_bounds = self.get_node(left_id).bounds.union(&self.get_node(right_id).bounds);
            
            let node = self.get_node_mut(node_id);
            node.height = new_height;
            node.bounds = new_bounds;
            
            node_id = node.parent;
        }
    }
}
