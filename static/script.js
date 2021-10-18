// socket initialize
const ws = new WebSocket("ws://localhost:3000/ws/");

ws.onopen = () => {
  ws.send(JSON.stringify({ message: "Hello!" }));
  console.log("ok");
  const obj = { key: 1, num: 300 };
  ws.send(JSON.stringify(obj));
};

ws.onmessage = (message) => {
  const _data = message.data;
  const data = JSON.parse(_data);
  if (data.type === "put white") {
    putRock(data.index, "white");
  }
  if (data.type === null) {
    console.log("data type is not defined");
  } else if (data.key === 1) {
    console.log("AI puts rock", typeof data);
  } else {
    console.log(data);
  }
};

//
const blocksElement = Array.from(document.getElementsByClassName("block"));
function submit() {
  console.log("submit");
  document.getElementById("body").innerHTML = new Date();
  ws.send("hello");
}

// start
let field = new Array(64);
field = field.map((el) => 0);

function clickfunc() {
  putRock(index, "black");
}

let clickLister;
function start() {
  blocksElement.map(function (el, index) {
    function clickfunc() {
      putRock(index, "black");
      sendPutEvent(index, "black");
    }
    clickLister = clickfunc;
    el.addEventListener("click", clickLister);
    putRock(locate(3, 3), "black");
    putRock(locate(4, 4), "black");
    putRock(locate(3, 4), "white");
    putRock(locate(4, 3), "white");
  });
}

// 石が置けるかチェック
function checkPuttable() {
  // if()
}

const rockType = {
  none: 0,
  white: 1,
  black: 2,
};

function putRock(index, color) {
  const blackEl = document.createElement("div");
  blackEl.classList.add(color);
  // 全部の石を削除
  while (blocksElement[index].hasChildNodes()) {
    blocksElement[index].removeChild(blocksElement[index].firstChild);
  }
  field[index] = rockType[color];
  blocksElement[index].removeEventListener("click", clickLister);
  blocksElement[index].appendChild(blackEl);
}

function sendPutEvent(index, color = "black") {
  const data = { index: index, color: color };
  sendServer(`put ${color}`, data);
}

function sendServer(type, data) {
  const obj = JSON.stringify({ type: type, data: data });
  ws.send(obj);
}

// util
function locate(row, column) {
  return row * 8 + column;
}
