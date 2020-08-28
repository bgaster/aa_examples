function OnParamChange(param, value) {
}

function controlChange(ctrlTag, value) {
}

function sendAttack(value) {
    sendMsg(1, 0, map(value, 0, 9, 0.001, 0.5))
}

function sendDecay(value) {
    sendMsg(1, 1, map(value, 0.0, 9.0, 0.001, 2.0))
}

function sendPitchMod(value) {
    sendMsg(1, 2, value)
}

function sendRelease(value) {
    sendMsg(1, 3, map(value, 0.0, 9.0, 0.001, 2.0))
}

function sendSustain(value) {
    sendMsg(1, 4, map(value, 0, 9, 0.1, 1.0))
}

function sendTremelo(value) {
    sendMsg(1, 5, value)
}

function sendVibrato(value) {
    sendMsg(1, 6, value)
}

function sendWaveform(value) {
    sendMsg(1, 7, value)
}

function map(x, in_min, in_max, out_min, out_max) {
    return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}