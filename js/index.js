const KEY_CODE_ENTER = 13;

window.onload = function() {
  const dom = {
    input: document.getElementById("input"),
    output: document.getElementById("output"),
  };

  import("../crate/pkg").then(module => {
    console.log("loaded module");
    dom.input.addEventListener("keypress", (evt) => {
      if (evt.keyCode === KEY_CODE_ENTER) {
        const input = dom.input.value;
        const output = module.tokens(input);
        dom.output.value = output;
      }
    });
  });
};
