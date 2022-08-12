const snazzyTheme = {
  foreground: "#eff0eb",
  background: "#282a36",
  selection: "#97979b33",
  black: "#282a36",
  brightBlack: "#686868",
  red: "#ff5c57",
  brightRed: "#ff5c57",
  green: "#5af78e",
  brightGreen: "#5af78e",
  yellow: "#f3f99d",
  brightYellow: "#f3f99d",
  blue: "#57c7ff",
  brightBlue: "#57c7ff",
  magenta: "#ff6ac1",
  brightMagenta: "#ff6ac1",
  cyan: "#9aedfe",
  brightCyan: "#9aedfe",
  white: "#f1f1f0",
  brightWhite: "#eff0eb"
};

const xterm = new window.Terminal({
  fontFamily: "\"Cascadia Code\", Menlo, monospace",
  theme: snazzyTheme,
  scrollback: 0
});
const fitAddon = new window.FitAddon.FitAddon();
xterm.loadAddon(fitAddon);

xterm.open(document.getElementById("terminal"));
fitAddon.fit();
try {
  const webgl = new window.WebglAddon.WebglAddon();
  xterm.loadAddon(webgl);
} catch (e) {
  console.warn("WebGL addon threw an exception during load", e);
}

const { master, slave } = openpty();
xterm.loadAddon(master);

const worker = new Worker("./worker.js");
new TtyServer(slave, false).start(worker);
