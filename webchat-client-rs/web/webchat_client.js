let app = null;
let socket = null;

export function run(currentApp) {
  app = currentApp;
  socket = new WebSocket('ws://127.0.0.1:8081');
  socket.binaryType = 'arraybuffer';
  socket.addEventListener('message', e => app.recv(new Uint8Array(e.data)));
  socket.addEventListener('open', e => app.main());

  document.getElementById("messageForm").addEventListener('submit', e => {
    e.preventDefault();
    let messageInput = document.getElementById("messageInput")
    app.input(messageInput.value);
    messageInput.value = "";
  });
}

export function send(msg) {
  socket.send(msg);
}

export function addMessage(msg) {
  let messages = document.getElementById('messages');
  let messageItem = document.createElement("li");
  messageItem.textContent = msg;
  messages.appendChild(messageItem);
  messages.scrollTop = messages.scrollTopMax;
}
