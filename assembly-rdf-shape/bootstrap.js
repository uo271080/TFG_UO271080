import './style.scss';
// import './node_modules/tailwindcss/tailwind.css'
import("./pkg").then(module => {
  module.run_app();
});
