const paramShape = 0
const paramSpread = 1
const paramLength = 2
const paramShimmer = 3

// Need to add midi learn 
const controlShape = 1
const controlSpread = 2
const controlLength = 3
const controlShimmer = 4
const controlDryWet = 5

function OnParamChange(param, value) {
    if (param == paramShape) {
        client.renderer.updateShape(value)
    }
    else if (param == paramSpread) {
        client.renderer.updateSpread(value)
    }
    else if (param == paramRelation) {
        client.renderer.updateLength(value)
    }
    else if (param == paramShimmer) {
        client.renderer.updateShimmer(value)
    }
}

function controlChange(ctrlTag, value) {
    if (ctrlTag == controlShape && client) {
        sendMsg(1, 3, client.renderer.controlShape(value))
    }
    else if (ctrlTag == controlSpread && client) {
        sendMsg(1, 5, client.renderer.controlSpread(value))
    }
    else if (ctrlTag == controlLength && client) {
        sendMsg(1, 1, client.renderer.controlLength(value))
    }
    else if (ctrlTag == controlShimmer && client) {
        sendMsg(1, 4, client.renderer.controlShimmer(value))
    }
    else if (ctrlTag == controlDryWet && client) {
        sendMsg(1, 0, client.renderer.controlDryWet(value))
    }
}