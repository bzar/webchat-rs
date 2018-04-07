let app = null;
let socket = null;

export function run(currentApp) {
  app = currentApp;
  socket = new WebSocket("ws://127.0.0.1:8081");
  socket.binaryType = "arraybuffer";
  socket.addEventListener('message', e => app.recv(new Uint8Array(e.data)));
  socket.addEventListener('open', e => app.main());
}

export function send(msg) {
  socket.send(msg);
}


