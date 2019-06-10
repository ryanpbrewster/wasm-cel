const KEY_CODE_ENTER = 13;

window.onload = function() {
  const dom = {
    input: document.getElementById("input"),
    value: document.getElementById("value"),
    ast: document.getElementById("ast"),
  };

  import("../crate/pkg").then(module => {
    console.log("loaded module");
    dom.input.addEventListener("keypress", (evt) => {
      if (evt.keyCode !== KEY_CODE_ENTER) {
        return;
      }
      if (evt.shiftKey) {
        dom.input.style.height = `${dom.input.scrollHeight}px`;
      } else {
        evt.preventDefault();
        const input = dom.input.value;

        const value = module.evaluate(input);
        dom.value.textContent = JSON.stringify(value, null, 2);

        const ast = module.parse_to_ast(input);
        dom.ast.textContent = JSON.stringify(ast, null, 2);
      }
    });
  });
};
