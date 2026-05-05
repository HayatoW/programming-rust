// 検証用の書き捨て

use self::BinaryTree::*;

fn main() {
    let jupiter_tree = NonEmpty(Box::new(TreeNone {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));
}

// `T` の順序付きコレクション
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNone<T>>),
}

// BinaryTree の一部
struct TreeNone<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}
