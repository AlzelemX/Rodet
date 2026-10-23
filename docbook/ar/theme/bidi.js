// In right-to-left text, "Rust++" shows as "++Rust": keep names that end in
// symbols left to right (outside code, which is already left to right).
(function () {
  var main = document.querySelector('main') || document.body;
  var walker = document.createTreeWalker(main, NodeFilter.SHOW_TEXT);
  var nodes = [];
  while (walker.nextNode()) nodes.push(walker.currentNode);
  nodes.forEach(function (node) {
    if (node.parentElement && node.parentElement.closest('pre, code')) return;
    node.data = node.data.replace(/(Rust\+\+|DEVR\+|C\+\+)/g, '⁦$1⁩');
  });
  document.querySelectorAll('.sidebar a, .menu-title').forEach(function (el) {
    el.childNodes.forEach(function (node) {
      if (node.nodeType === 3) node.data = node.data.replace(/(Rust\+\+|DEVR\+)/g, '⁦$1⁩');
    });
  });
})();
