extern crate core;

#[cfg(test)]
mod tests;

use crate::math::{Bounds, Vec2};
use crate::util::pool;
use crate::world::{Bodies, ConstraintsMap, BodyPair, Body, BodyId};
use crate::collision::ContactConstraint;

use std;
use crate::collision::broad_phase::{BroadPhase, ProxyId};

trait Nullable {
    const NULL: Self;
}

type NodeId = pool::PoolId;

impl Nullable for NodeId {
    const NULL: NodeId = std::usize::MAX;
}

#[derive(Clone)]
struct Node<T: Default> {
    bounds: Bounds,
    
    parent: NodeId,
    left: NodeId,
    right: NodeId,
    
    data: T,
    
    height: u32,
}

impl<T: Default> Default for Node<T> {
    fn default() -> Node<T> {
        Node {
            bounds: Bounds::new(Vec2::ZERO, Vec2::ZERO),
            parent: NodeId::NULL,
            left: NodeId::NULL,
            right: NodeId::NULL,
            data: T::default(),
            height: 0,
        }
    }
}

impl<T: Default> Node<T> {
    fn new(bounds: Bounds, data: T) -> Node<T> {
        Node {
            bounds,
            parent: NodeId::NULL,
            left: NodeId::NULL,
            right: NodeId::NULL,
            data,
            height: 0,
        }
    }
    
    fn is_leaf(&self) -> bool {
        self.left == NodeId::NULL
    }
}

struct BoundsTree<T: Default> {
    pool: pool::Pool<Node<T>>,
    root_id: NodeId,
}

impl<T: Default> BoundsTree<T> {
    pub fn new() -> BoundsTree<T> {
        let pool = pool::Pool::default();
        BoundsTree {
            root_id: NodeId::NULL,
            pool,
        }
    }
    
    pub fn get_root(&self) -> &Node<T> {
        self.get_node(self.root_id)
    }
    
    pub fn get_root_mut(&mut self) -> &mut Node<T> {
        let root_id = self.root_id;
        self.get_node_mut(root_id)
    }
    
    pub fn get_node(&self, node_id: NodeId) -> &Node<T> {
        self.pool.get(node_id)
    }
    
    pub fn get_node_mut(&mut self, node_id: NodeId) -> &mut Node<T> {
        self.pool.get_mut(node_id)
    }
    
    /// Updates the height and bounds of a branch node.
    fn update_branch_node(&mut self, node_id: NodeId) {
        let (new_height, new_bounds) = {
            let node = self.get_node(node_id);
    
            let left = self.get_node(node.left);
            let right = self.get_node(node.right);
    
            let new_height = 1 + left.height.max(right.height);
            let new_bounds = left.bounds.union(&right.bounds);
            
            (new_height, new_bounds)
        };
    
        let node = self.get_node_mut(node_id);
        node.height = new_height;
        node.bounds = new_bounds;
    }
    
    /// Rotates the subtree rooted at `node_id` so that it is rooted at `child_id`. After this
    /// operation, the subtree will be rooted at `child_id`.
    fn rotate(&mut self, node_id: NodeId, child_id: NodeId) {
        let (is_left_child, node_parent_id) = {
            let node = self.get_node(node_id);
            assert!(node.left == child_id || node.right == child_id);
            (node.left == child_id, node.parent)
        };
        
        let (grandchild_left_id, grandchild_right_id) = {
            let child = self.get_node(child_id);
            (child.left, child.right)
        };
        
        // Swap node and child
        {
            let child = self.get_node_mut(child_id);
            child.parent = node_parent_id;
            // It doesn't matter which child of `child` `node` is
            child.left = node_id;
        }
    
        self.get_node_mut(node_id).parent = child_id;
    
        // Replace `node` with `child` in child's (formerly node's) parent
        if self.root_id == node_id {
            self.root_id = child_id;
        } else {
            let node_parent = self.get_node_mut(node_parent_id);
            
            if node_parent.left == node_id {
                node_parent.left = child_id;
            } else {
                debug_assert_eq!(node_parent.right, node_id);
                node_parent.right = child_id;
            }
        }
        
        // Place the shallower grandchild as a child of `node`, replacing `child`
        // child.right becomes the deeper grandchild (child.left == node)
        // If both grandchildren are equally deep, move the left to node and retain the right
        if self.get_node(grandchild_left_id).height <= self.get_node(grandchild_right_id).height {
            if is_left_child {
                self.get_node_mut(node_id).left = grandchild_left_id;
            } else {
                self.get_node_mut(node_id).right = grandchild_left_id;
            }
            
            self.get_node_mut(grandchild_left_id).parent = node_id;
            self.get_node_mut(child_id).right = grandchild_right_id;
        } else {
            if is_left_child {
                self.get_node_mut(node_id).left = grandchild_right_id;
            } else {
                self.get_node_mut(node_id).right = grandchild_right_id;
            }
            
            self.get_node_mut(grandchild_right_id).parent = node_id;
            self.get_node_mut(child_id).right = grandchild_left_id;
        }
        
        self.update_branch_node(node_id);
        self.update_branch_node(child_id);
    }
    
    /// Balances the subtree rooted at `node_id` and returns the `NodeId` of the new subtree root.
    fn balance_subtree(&mut self, node_id: NodeId) -> NodeId {
        if self.get_node(node_id).height < 2 {
            return node_id;
        }
    
        let (left_id, right_id) = {
            let node = self.get_node(node_id);
            (node.left, node.right)
        };
        
        let height_diff = self.get_node(right_id).height as i32 - self.get_node(left_id).height as i32;
        
        if height_diff > 1 {
            self.rotate(node_id, right_id);
            right_id
        } else if height_diff < -1 {
            self.rotate(node_id, left_id);
            left_id
        } else {
            // Already balanced
            node_id
        }
    }

    /// Walks up the tree from `node_id`, balancing subtrees and fixing ancestor heights and bounds.
    fn update_ancestors(&mut self, mut node_id: NodeId) {
        while node_id != NodeId::NULL {
            node_id = self.balance_subtree(node_id);
            
            self.update_branch_node(node_id);
            
            node_id = self.get_node(node_id).parent;
        }
    }
    
    /// Inserts a leaf node with the given `data` into the tree.
    ///
    /// When inserting the leaf node in a new branch with an existing leaf, the existing leaf is
    /// made the `left` child of the new branch, and the new leaf node is made the `right` child.
    ///
    /// # Returns
    /// The `NodeId` of the inserted leaf node.
    fn insert_leaf(&mut self, bounds: Bounds, data: T) -> NodeId {
        let leaf_id = self.pool.allocate_with(Node::new(bounds, data));
        
        if self.root_id == NodeId::NULL {
            self.root_id = leaf_id;
            return leaf_id;
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
    
        let (sibling_parent_id, sibling_bounds, sibling_height) = {
            let sibling = self.get_node(sibling_id);
            (sibling.parent, sibling.bounds, sibling.height)
        };
        
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
        
        let parent_id = self.get_node(leaf_id).parent;
        self.update_ancestors(parent_id);
        
        leaf_id
    }
    
    /// Removes the leaf identified by `leaf_id` from the tree.
    ///
    /// The removal process involves replacing the parent of the leaf with its sibling.
    fn remove_leaf(&mut self, leaf_id: NodeId) {
        if self.root_id == leaf_id {
            self.root_id = NodeId::NULL;
            self.pool.free(leaf_id);
            return;
        }
        
        let parent_id = self.get_node(leaf_id).parent;
        let sibling_id = {
            let parent = self.get_node(parent_id);
            if parent.left == leaf_id {
                parent.right
            } else {
                parent.left
            }
        };
        
        if self.root_id == parent_id {
            // Root is parent, replace root
            
            self.pool.free(parent_id);
            
            self.root_id = sibling_id;
            self.get_node_mut(sibling_id).parent = NodeId::NULL;
            
            self.pool.free(leaf_id);
            
            return;
        }
    
        // Delete parent and connect sibling to grandparent in its place
        
        let grandparent_id = self.get_node(parent_id).parent;
    
        let is_parent_left_child = self.get_node(grandparent_id).left == parent_id;
        
        self.pool.free(parent_id);
    
        if is_parent_left_child {
            self.get_node_mut(grandparent_id).left = sibling_id;
        } else {
            self.get_node_mut(grandparent_id).right = sibling_id;
        }
        
        self.get_node_mut(sibling_id).parent = grandparent_id;
        
        self.update_ancestors(grandparent_id);
        
        self.pool.free(leaf_id);
    }
    
    fn query<F>(&self, bounds: Bounds, mut f: F)
        where F: FnMut(&Node<T>) -> bool {
        let mut stack = Vec::with_capacity(self.pool.object_count);
        stack.push(self.root_id);
    
        while stack.len() > 0 {
            let node_id = stack.pop().unwrap();
            let node = self.get_node(node_id);
            
            if !bounds.intersects(&node.bounds) {
                continue;
            }
            
            if node.is_leaf() {
                if !f(node) {
                    return;
                }
            } else {
                stack.push(node.left);
                stack.push(node.right);
            }
        }
    }
}

pub struct BoundsTreeBroadPhase {
    tree: BoundsTree<BodyId>,
    reinserted_bodies: Vec<BodyId>,
}

impl BoundsTreeBroadPhase {
    pub fn new() -> BoundsTreeBroadPhase {
        BoundsTreeBroadPhase {
            tree: BoundsTree::new(),
            reinserted_bodies: Vec::new(),
        }
    }
    
    pub fn post_update(&mut self) {
        self.reinserted_bodies.clear();
    }
}

const EXPANSION_FACTOR: f32 = 0.05;

impl BroadPhase for BoundsTreeBroadPhase {
    fn new_potential_pairs(&self, bodies: &Bodies,
                           constraints: &mut ConstraintsMap<ContactConstraint>) {
        for body_id in self.reinserted_bodies.iter() {
            let body = &bodies[*body_id];
            
            if body.is_static() {
                continue;
            }
            
            self.tree.query(body.bounds, |node| {
                if node.data == *body_id {
                    return true;
                }
                
                let body_pair = BodyPair::new(node.data, *body_id);
                if !constraints.contains_key(&body_pair) {
                    constraints.insert(body_pair, Vec::new());
                }
                true
            });
        }
    }
    
    fn create_proxy(&mut self, body: &Body) -> ProxyId {
        self.tree.insert_leaf(body.bounds.expand_by(EXPANSION_FACTOR), body.id)
    }
    
    fn destroy_proxy(&mut self, proxy_id: ProxyId) {
        self.tree.remove_leaf(proxy_id);
    }
    
    fn update_proxy(&mut self, proxy_id: ProxyId, body: &Body) {
        // TODO: Explore rotation based method instead
        
        if self.tree.get_node(proxy_id).bounds.contains(&body.bounds) {
            return;
        }
        
        // Proxy (node) id is guaranteed to remain the same after the reinsert
        
        self.destroy_proxy(proxy_id);
        
        self.create_proxy(body);
        
        self.reinserted_bodies.push(body.id);
    }
}
