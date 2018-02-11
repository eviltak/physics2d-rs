use super::*;

#[test]
fn insert_leaf() {
    let mut tree = BoundsTree::new();
    let (a, b, c) = (tree.pool.allocate(), tree.pool.allocate(), tree.pool.allocate());
    
    // Upon insertion, b and c will be siblings.
    // The tree should have the following structure:
    //          root
    //         /    \
    //        a   (branch)
    //             /    \
    //            b      c
    
    tree.get_node_mut(a).bounds = Bounds::new(Vec2::ZERO, Vec2::ONE * 3.0);
    tree.get_node_mut(b).bounds = Bounds::new(Vec2::RIGHT * 4.0, Vec2::RIGHT * 4.0 + Vec2::ONE);
    tree.get_node_mut(c).bounds = Bounds::new(Vec2::RIGHT * 4.0 + Vec2::UP * 1.01,
                                              Vec2::RIGHT * 4.0 + Vec2::UP * 1.01 + Vec2::ONE);
    
    tree.insert_leaf(a);
    
    assert_eq!(tree.root_id, a);
    
    tree.insert_leaf(b);
    
    assert_ne!(tree.root_id, a);
    
    assert_eq!(tree.get_root().left, a);
    assert_eq!(tree.get_root().right, b);
    
    tree.insert_leaf(c);
    
    let root = tree.get_root();
    
    println!("{:?}", root.bounds.perimeter());
    
    // left (a) unchanged, right made branch and b pushed down
    assert_eq!(root.left, a);
    assert_ne!(root.right, b);
    
    let right = tree.get_node(root.right);
    
    assert_eq!(right.left, b);
    assert_eq!(right.right, c);
}