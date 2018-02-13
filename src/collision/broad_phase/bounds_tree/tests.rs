use super::*;

#[test]
fn insert_leaf() {
    let mut tree = BoundsTree::new();
    
    // Upon insertion, b and c will be siblings.
    // The tree should have the following structure:
    //          root
    //         /    \
    //        a   (branch)
    //             /    \
    //            b      c
    
    let bounds_a = Bounds::new(Vec2::ZERO, Vec2::ONE * 3.0);
    let bounds_b = Bounds::new(Vec2::RIGHT * 4.0, Vec2::RIGHT * 4.0 + Vec2::ONE);
    let bounds_c = Bounds::new(Vec2::RIGHT * 4.0 + Vec2::UP * 1.01,
                               Vec2::RIGHT * 4.0 + Vec2::UP * 1.01 + Vec2::ONE);
    
    let a = tree.insert_leaf(bounds_a, 0);
    
    assert_eq!(tree.root_id, a);
    
    let b = tree.insert_leaf(bounds_b, 1);
    
    assert_ne!(tree.root_id, a);
    
    assert_eq!(tree.get_root().left, a);
    assert_eq!(tree.get_root().right, b);
    
    let c = tree.insert_leaf(bounds_c, 2);
    
    let root = tree.get_root();
    
    // left (a) unchanged, right made branch and b pushed down
    assert_eq!(root.left, a);
    assert_ne!(root.right, b);
    
    let right = tree.get_node(root.right);
    
    assert_eq!(right.left, b);
    assert_eq!(right.right, c);
}

#[test]
fn remove_leaf() {
    let mut tree = BoundsTree::new();
    
    // Upon insertion, b and c will be siblings.
    // The tree should have the following structure:
    //          root
    //         /    \
    //        a   (branch)
    //             /    \
    //            b      c
    
    let bounds_a = Bounds::new(Vec2::ZERO, Vec2::ONE * 3.0);
    let bounds_b = Bounds::new(Vec2::RIGHT * 4.0, Vec2::RIGHT * 4.0 + Vec2::ONE);
    let bounds_c = Bounds::new(Vec2::RIGHT * 4.0 + Vec2::UP * 1.01,
                               Vec2::RIGHT * 4.0 + Vec2::UP * 1.01 + Vec2::ONE);
    
    let a = tree.insert_leaf(bounds_a, 0);
    let b = tree.insert_leaf(bounds_b, 1);
    let c = tree.insert_leaf(bounds_c, 2);
    
    assert_ne!(tree.get_root().right, c);
    
    // On deleting b:
    //          root
    //         /    \
    //        a      c
    
    tree.remove_leaf(b);
    
    assert_eq!(tree.get_root().right, c);
    
    // On deleting c:
    //          root == a
    
    tree.remove_leaf(c);
    
    assert_eq!(tree.root_id, a);
}
