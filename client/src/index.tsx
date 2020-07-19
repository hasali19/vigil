import React from "react";
import ReactDOM from "react-dom";

import { createMuiTheme, ThemeProvider } from "@material-ui/core";
import { blue, amber } from "@material-ui/core/colors";

import "./index.css";
import App from "./App";

const theme = createMuiTheme({
  palette: {
    primary: blue,
    secondary: amber,
  },
});

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <App />
  </ThemeProvider>,
  document.getElementById("root")
);
