// import YASHE from "/node_modules/yashe/dist/yashe.bundled.min";
// import "yashe/dist/yashe.min.css";

window.onload = function() {
  const divRef = document.getElementById('editor');

  let yashe = null;

  const options = {
    persistent: false,
    lineNumbers: true
  };

  if (!yashe) {
    yashe = YASHE(divRef, options);
    yashe.refresh();
  }
}