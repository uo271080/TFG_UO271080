import YASHE from "yashe/dist/yashe.bundled.min";
import "yashe/dist/yashe.min.css";

function initYASHE(elementId) {
  const yashe = YASHE.fromTextArea(document.getElementById(elementId), {
      // Configuraciones de YASHE
  });
  return yashe;
}

module.exports =  initYASHE;