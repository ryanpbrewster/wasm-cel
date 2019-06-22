const KEY_CODE_ENTER = 13;
const KEY_CODE_BACKSPACE = 8;

window.onload = function() {
  const dom = {
    input: document.getElementById("input"),
    ast: document.getElementById("ast"),
  };

  function autosize(element) {
    element.style.height = "auto";
    element.style.height = element.scrollHeight + "px";
  }
  autosize(dom.input);
  dom.input.addEventListener("keypress", (evt) => autosize(dom.input));
  dom.input.addEventListener("keyup", (evt) => {
    if (evt.keyCode === KEY_CODE_BACKSPACE) {
      autosize(dom.input);
    }
  });

  import("../crate/pkg").then(module => {
    dom.input.addEventListener("keypress", (evt) => {
      if (evt.keyCode !== KEY_CODE_ENTER) {
        return;
      }
      if (evt.shiftKey) {
        dom.input.style.height = `${dom.input.scrollHeight}px`;
      } else {
        evt.preventDefault();
        const input = dom.input.value;

        const ast = module.parse_to_ast(input);
        dom.ast.textContent = JSON.stringify(ast, null, 2);

        const output = module.process(input);
        constructD3Ast(output);
      }
    });
  });


  function constructD3Ast(treeData) {
    // Set the dimensions and margins of the diagram
    var margin = {top: 20, right: 90, bottom: 30, left: 90},
        width = 1200 - margin.left - margin.right,
        height = 500 - margin.top - margin.bottom;

    d3.select("#graph").select("svg").remove();
    var svg = d3.select("#graph").append("svg")
        .attr("width", width + margin.right + margin.left)
        .attr("height", height + margin.top + margin.bottom)
      .append("g")
        .attr("transform", "translate("
              + margin.left + "," + margin.top + ")");

    var i = 0,
        duration = 750,
        root;

    // declares a tree layout and assigns the size
    var treemap = d3.tree().size([height, width]);

    // Assigns parent, children, height, depth
    root = d3.hierarchy(treeData, function(d) { return d.children; });
    root.x0 = height / 2;
    root.y0 = 0;

    update(root);

    // Collapse the node and all it's children
    function collapse(d) {
      if(d.children) {
        d._children = d.children
        d._children.forEach(collapse)
        d.children = null
      }
    }

    function update(source) {
      // Assigns the x and y position for the nodes
      var treeData = treemap(root);

      // Compute the new tree layout.
      var nodes = treeData.descendants(),
          links = treeData.descendants().slice(1);

      // Normalize for fixed-depth.
      nodes.forEach(function(d){ d.y = d.depth * 180});

      // ****************** Nodes section ***************************

      // Update the nodes...
      var node = svg.selectAll('g.node')
          .data(nodes, function(d) {return d.id || (d.id = ++i); });

      // Enter any new modes at the parent's previous position.
      var nodeEnter = node.enter().append('g')
          .attr('class', 'node')
          .attr("transform", function(d) {
            return "translate(" + source.y0 + "," + source.x0 + ")";
        })
        .on('click', click);

      // Add Circle for the nodes
      nodeEnter.append('circle')
          .attr('class', 'node');

      // Add labels for the nodes
      nodeEnter.append('text')
          .attr("dy", ".35em")
          .attr("x", function(d) {
              return d.children || d._children ? -13 : 13;
          })
          .attr("text-anchor", function(d) {
              return d.children || d._children ? "end" : "start";
          });

      // UPDATE
      var nodeUpdate = nodeEnter.merge(node);

      // Transition to the proper position for the node
      nodeUpdate.transition()
        .duration(duration)
        .attr("transform", function(d) { 
            return "translate(" + d.y + "," + d.x + ")";
         });

      // Update the node attributes and style
      nodeUpdate.select('circle.node')
        .attr('r', 10)
        .style("fill", function(d) {
            const color = d.data.result["Err"] ? "red" : "blue";
            if (d.children) {
              // Expanded parents are light
              return "light" + color;
            }
            if (d._children) {
              // Collapsed parents are bright
              return "dark" + color;
            }
            // primitives are plain white
            return "white";
        })
        .attr('cursor', 'pointer');

      nodeUpdate.select('text')
         .text((d) => d.children ? extractOp(d.data.op) : extractResult(d.data.result));


      // Remove any exiting nodes
      var nodeExit = node.exit().transition()
          .duration(duration)
          .attr("transform", function(d) {
              return "translate(" + source.y + "," + source.x + ")";
          })
          .remove();

      // On exit reduce the node circles size to 0
      nodeExit.select('circle')
        .attr('r', 1e-6);

      // On exit reduce the opacity of text labels
      nodeExit.select('text')
        .style('fill-opacity', 1e-6);

      // ****************** links section ***************************

      // Update the links...
      var link = svg.selectAll('path.link')
          .data(links, function(d) { return d.id; });

      // Enter any new links at the parent's previous position.
      var linkEnter = link.enter().insert('path', "g")
          .attr("class", "link")
          .attr('d', function(d){
            var o = {x: source.x0, y: source.y0}
            return diagonal(o, o)
          });

      // UPDATE
      var linkUpdate = linkEnter.merge(link);

      // Transition back to the parent element position
      linkUpdate.transition()
          .duration(duration)
          .attr('d', function(d){ return diagonal(d, d.parent) });

      // Remove any exiting links
      var linkExit = link.exit().transition()
          .duration(duration)
          .attr('d', function(d) {
            var o = {x: source.x, y: source.y}
            return diagonal(o, o)
          })
          .remove();

      // Store the old positions for transition.
      nodes.forEach(function(d){
        d.x0 = d.x;
        d.y0 = d.y;
      });

      // Creates a curved (diagonal) path from parent to the child nodes
      function diagonal(s, d) {

        path = `M ${s.y} ${s.x}
                C ${(s.y + d.y) / 2} ${s.x},
                  ${(s.y + d.y) / 2} ${d.x},
                  ${d.y} ${d.x}`

        return path
      }

      // Toggle children on click.
      function click(d) {
        if (d.children) {
          d._children = d.children;
          d.children = null;
        } else {
          d.children = d._children;
          d._children = null;
        }
        update(d);
      }
    }
  }
};

function extractResult(result) {
  const ok = result["Ok"];
  if (!ok) {
    return JSON.stringify(result["Err"]);
  }
  return extractValue(ok);
}

function extractValue(ok) {
  console.log(ok);
  switch (ok["t"]) {
  case "List":
    return "[" + ok["c"].map(v => extractValue(v)).join(", ") + "]";

  case "Map":
    return "{" + Object.entries(ok["c"]).map(([k, v]) => `"${k}" : ${extractValue(v)}`).join(", ") + "}";

  case "String":
    return JSON.stringify(ok["c"]);

  case "Null":
    return "null";

  default:
    return JSON.stringify(ok["c"]);
  }
}

function extractOp(op) {
  switch (op.t) {
    case "Member":
      return `.${op.c}`;
    case "Method":
      return `.${op.c}()`;
    default:
      return op.t;
  }
}
