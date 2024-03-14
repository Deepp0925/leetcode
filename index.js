var connect = function (root) {
  if (!root) return root;

  if (root.left) {
    if (root.right) {
      root.left.next = root.right;
    } else {
      let next = root.next;
      while (next) {
        root.left.next = next?.left || next?.right || null;
        next = next.next;
        if (root.left.next) break;
      }
    }
  }

  if (root.right) {
    let next = root.next;
    while (next) {
      root.right.next = next?.left || next?.right || null;
      next = next.next;
      if (root.right.next) break;
    }
  }

  connect(root.right);
  connect(root.left);

  return root;
};
