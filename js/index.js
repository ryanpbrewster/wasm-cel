const KEY_CODE_ENTER = 13;

window.onload = function() {
  const dom = {
    input: document.getElementById("input"),
    output: document.getElementById("output"),
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
        const output = module.parse_to_ast(input);
        dom.output.textContent = JSON.stringify(output, null, 2);
      }
    });
  });
};
