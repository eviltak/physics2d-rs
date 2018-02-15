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

#[test]
fn balance() {
    let mut tree = BoundsTree::new();
    
    let bounds_a = Bounds::new(Vec2::ZERO, Vec2::ONE * 3.0);
    let bounds_b = Bounds::new(Vec2::RIGHT * 4.0, Vec2::RIGHT * 4.0 + Vec2::ONE);
    let bounds_c = Bounds::new(Vec2::RIGHT * 4.0 + Vec2::UP * 1.01,
                               Vec2::RIGHT * 4.0 + Vec2::UP * 1.01 + Vec2::ONE);
    let bounds_d = Bounds::new(Vec2::RIGHT * 4.0 + Vec2::UP * 2.01,
                               Vec2::RIGHT * 4.0 + Vec2::UP * 2.01 + Vec2::ONE);
    
    let a = tree.insert_leaf(bounds_a, 0);
    let b = tree.insert_leaf(bounds_b, 1);
    
    let root_id = tree.get_node(a).parent;
    
    let c = tree.insert_leaf(bounds_c, 2);
    
    let branch0_id = tree.get_node(c).parent;
    
    let d = tree.insert_leaf(bounds_d, 3);
    
    let branch1_id = tree.get_node(d).parent;
    
    // The tree -should- have the following structure:
    //          root
    //         /    \
    //        a    branch0
    //             /     \
    //            b    branch1
    //                 /    \
    //                c      d
    //
    // `update_ancestors` will ensure that the tree is balanced to:
    //
    //               branch0
    //              /       \
    //            root    branch1
    //           /    \    /   \
    //          a      b  c     d
    
    assert_eq!(tree.root_id, branch0_id);
    
    let root = tree.get_root();
    
    assert_eq!(root.left, root_id);
    assert_eq!(root.right, branch1_id);
    
    let (left, right) = (tree.get_node(root.left), tree.get_node(root.right));
    
    assert_eq!(left.left, a);
    assert_eq!(left.right, b);
    assert_eq!(right.left, c);
    assert_eq!(right.right, d);
}
