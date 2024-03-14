var connect = function (root) {
  if (!root?.left) return root;
  root.left.next = root.right;
  root.right.next = root.next ? root.next.left : null;

  connect(root.left);
  connect(root.right);

  return root;
};
