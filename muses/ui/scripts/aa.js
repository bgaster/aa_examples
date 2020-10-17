// This currently only works with the Push 2's endless encoders. We should really 
// add MIDI learn and the like, but that is for another rainy day

// TODO: Midi learn
// TODO: move to class so that vars encapuslated 

const paramFilter = 0
const paramWave = 1
const paramRelation = 2
const paramSub = 3

// Need to add midi learn 
const controlFilter = 15
const controlWave = 71
const controlRelation = 72
const controlSub = 73

const controlAttack = 74
const controlDecay = 75
const controlSubstain = 76
const controlRelease = 77

const nukeNode = 0
const reflectNode = 1

const paramShape = 0
const paramSpread = 1
const paramLength = 2
const paramShimmer = 3

// Need to add midi learn 
const controlShape = 15
const controlSpread = 71
const controlLength = 72
const controlShimmer = 73
const controlDryWet = 74

var controlNuke = true

function OnParamChange(node, param, value) {
    if (node == nukeNode) {
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
    else if (node == reflectNode) {
        if (param == paramShape) {
            client.reflectRenderer.updateShape(value)
        }
        else if (param == paramSpread) {
            client.reflectRenderer.updateSpread(value)
        }
        else if (param == paramRelation) {
            client.reflectRenderer.updateLength(value)
        }
        else if (param == paramShimmer) {
            client.reflectRenderer.updateShimmer(value)
        }
    }
}

function controlChange(ctrlTag, value) {
    //console.log(ctrlTag + " " + value)

    if (ctrlTag == 20 && value == 127) {
        controlNuke = !controlNuke
        return
    }

    // Endless controller on Push 2 is 1 for +1 (turn clockwise) and 127 for -1 (turn anti-clockwise)
    if (value == 127) {
        value = -1
    }

    if (controlNuke) {
        if (ctrlTag == controlFilter && client) {
            parent.sendMsg(1, 0, 0, client.renderer.incFilter(value))
        }
        else if (ctrlTag == controlWave && client) {
            parent.sendMsg(1, 0, 3, client.renderer.incWave(value))
        }
        else if (ctrlTag == controlRelation && client) {
            parent.sendMsg(1, 0, 1, client.renderer.incRelation(value))
        }
        else if (ctrlTag == controlSub && client) {
            parent.sendMsg(1, 0, 2, client.renderer.incSub(value))
        }
        else if (ctrlTag == controlAttack && client) {
            parent.sendMsg(1, 0, 4, client.adsr.incAttack(value))
        }
        else if (ctrlTag == controlDecay && client) {
            parent.sendMsg(1, 0, 5, client.adsr.incDecay(value))
        }
        else if (ctrlTag == controlSubstain && client) {
            parent.sendMsg(1, 0, 7, client.adsr.incSubstain(value))
        }
        else if (ctrlTag == controlRelease && client) {
            parent.sendMsg(1, 0, 6, client.adsr.incRelease(value))
        }
    }
    else {
        // reflect midl

        if (ctrlTag == controlShape && client) {
            parent.sendMsg(1, 1, 3, client.reflectRenderer.incShape(value))
        }
        else if (ctrlTag == controlSpread && client) {
            parent.sendMsg(1, 1, 5, client.reflectRenderer.incSpread(value))
        }
        else if (ctrlTag == controlLength && client) {
            parent.sendMsg(1, 1, 1, client.reflectRenderer.incLength(value))
        }
        else if (ctrlTag == controlShimmer && client) {
            parent.sendMsg(1, 1, 4, client.reflectRenderer.incShimmer(value))
        }
        else if (ctrlTag == controlDryWet && client) {
            parent.sendMsg(1, 1, 0, client.reflectRenderer.incDryWet(value))
        }
    }
    
}