var console_msg = 0;
var send_param = 1;
var get_param = 2;
var change_module = 3;

function getParam(paramIdx) {
    var message = {
      "msg": "getParam",
      "index": paramIdx,
    };
    external.invoke(JSON.stringify(message));
}

function sendParam(paramIdx, value) {
    var message = {
      "msg": "sendParam",
      "index": paramIdx,
      "value": value
    };
    external.invoke(JSON.stringify(message));
}

function sendMsg(msgType, node, paramIdx, value) {
  if (value == undefined) {
    var message = {
      "msg": msgType,
      "node": node,
      "index": paramIdx,
    };
  }
  else {
    var message = {
      "msg": msgType,
      "node": node,
      "index": paramIdx,
      "value": value
    };
  }
  external.invoke(JSON.stringify(message));
}

function sendConsole(value) {
  var message = {
    "msg": "console",
    "node": 0,
    "index": 0,
    "value": value
  };
  external.invoke(JSON.stringify(message));
}