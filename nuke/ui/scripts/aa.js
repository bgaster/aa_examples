const paramFilter = 0
const paramWave = 1
const paramRelation = 2
const paramSub = 3

// Need to add midi learn 
const controlFilter = 1
const controlWave = 2
const controlRelation = 3
const controlSub = 4

const controlAttack = 5
const controlDecay = 6
const controlSubstain = 7
const controlRelease = 8

function OnParamChange(param, value) {
    if (param == paramFilter) {
        client.renderer.updateFilter(value)
    }
    else if (param == paramWave) {
        client.renderer.updateWave(value)
    }
    else if (param == paramRelation) {
        client.renderer.updateRelation(value)
    }
    else if (param == paramSub) {
        client.renderer.updateSub(value)
    }
}

function controlChange(ctrlTag, value) {
    if (ctrlTag == controlFilter && client) {
        sendMsg(1, 0, client.renderer.controlFilter(value))
    }
    else if (ctrlTag == controlWave && client) {
        sendMsg(1, 3, client.renderer.controlWave(value))
    }
    else if (ctrlTag == controlRelation && client) {
        sendMsg(1, 1, client.renderer.controlRelation(value))
    }
    else if (ctrlTag == controlSub && client) {
        sendMsg(1, 2, client.renderer.controlSub(value))
    }
    else if (ctrlTag == controlAttack && client) {
        sendMsg(1, 4, client.adsr.controlAttack(value))
    }
    else if (ctrlTag == controlDecay && client) {
        sendMsg(1, 5, client.adsr.controlDecay(value))
    }
    else if (ctrlTag == controlSubstain && client) {
        sendMsg(1, 7, client.adsr.controlSubstain(value))
    }
    else if (ctrlTag == controlRelease && client) {
        sendMsg(1, 6, client.adsr.controlRelease(value))
    }
}