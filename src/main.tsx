import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import initFishWasm from "fish-wasm";
import "./index.css";

initFishWasm().then(() => {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
});
