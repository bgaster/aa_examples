/*
 * 
 */
'use strict'

function ADSR (client) {
    var attack = 2.001
    var attackMin = 0.001
    var attackMax = 4.0
    var attackStep = 0.01

    var decay = 1.0
    var decayMin = 0.0
    var decayMax = 4.0
    var decayStep = 0.01
    
    var substain = 0.75
    var substainMin = 0.0
    var substainMax = 1.0
    var substainStep = 0.01
    
    var release = 2.0
    var releaseMin = 0.0
    var releaseMax = 4.0
    var releaseStep = 0.01

    const red = "#DD4A22"
    const yellow = "#F3B83C"
    const blue = "#6060A6"
    const rose = "#FFBDB0"
    const rootbeer = "#1f0c07"

    this.controlAttack = (v) => {
        let mv = map(v, 0, 127, attackMin, attackMax)
        attack = mv
        return mv
    }

    this.controlDecay = (v) => {
        let mv = map(v, 0, 127, decayMin, decayMax)
        decay = mv
        return mv
    }
    
    this.controlSubstain = (v) => {
        let mv = map(v, 0, 127, substainMin, substainMax)
        substain = mv
        return mv 
    }
    
    this.controlRelease = (v) => {
        let mv = map(v, 0, 127, releaseMin, releaseMax)
        release = mv
        return mv
    }

    this.start = function () {
        console.log("ADSR")
    }

    this.draw = function(context, transpose) {
        drawEnvelope(context, transpose)
    }

    drawEnvelope = function(context, transpose) {
        let bX = 30.0
        let bY = 60.0
        let bWidth = 260.0
        let bHeight = 110.0
        
        let spacing = 10.0
        let maxWidth = (bWidth - 3.0 * spacing) / 3.0
        
        let aw = maxWidth * normalize(attack, attackMin, attackMax) 
        let dw = maxWidth * normalize(decay, decayMin, decayMax)
        let sh = bHeight * normalize(substain, substainMin, substainMax)
        let rw = maxWidth * normalize(release, releaseMin, releaseMax)
        let arcSize = 0.9

        let moveTo = (p) => { 
            let tp = translate(transpose, p)
            context.moveTo(tp.x, tp.y)
        }
        let lineTo = (p) => {
            let tp = translate(transpose, p)
            context.lineTo(tp.x, tp.y)
        }

        let bezierCurveTo = (p1, p2, p3) => {
            let tp1 = translate(transpose, p1)
            let tp2 = translate(transpose, p2)
            let tp3 = translate(transpose, p3)
            context.bezierCurveTo(tp1.x, tp1.y, tp2.x, tp2.y, tp3.x, tp3.y)
        }

        let quadraticCurveTo = (p1, p2) => {
            let tp1 = translate(transpose, p1)
            let tp2 = translate(transpose, p2)
            context.quadraticCurveTo(tp1.x, tp1.y, tp2.x, tp2.y)
        }

        // line
        context.beginPath()
        moveTo(point(bX, bY + bHeight + spacing))
        lineTo(point(bX + bWidth, bY + bHeight + spacing))
        context.strokeStyle = "#d3d3d3"
        context.stroke()

        // attack
        context.beginPath()
        moveTo(point(bX, bY + bHeight))
        quadraticCurveTo(point(bX + aw * arcSize, bY + bHeight * arcSize), point(bX + aw, bY))
        lineTo(point(bX + aw, bY + bHeight))
        lineTo(point(bX, bY + bHeight))
        context.strokeStyle = rose
        context.stroke()

        context.beginPath()
        moveTo(point(bX, bY + bHeight));
        quadraticCurveTo(point(bX + aw * arcSize, bY + bHeight * arcSize), point(bX + aw, bY));
        lineTo(point(bX + aw, bY + bHeight))
        context.fillStyle = rose
        context.fill()

        // decay        
        context.beginPath()
        moveTo(point(bX + aw + spacing, bY + bHeight))
        lineTo(point(bX + aw + spacing, bY))
        quadraticCurveTo(point(
            bX + aw + spacing + dw * (1.0 - arcSize), bY + (bHeight - sh) * arcSize), 
            point(bX + aw + spacing + dw, bY + bHeight - sh))
        lineTo(point(bX + aw + spacing + dw, bY + bHeight))
        context.closePath()
        context.strokeStyle = blue
        context.stroke()

        context.beginPath()
        moveTo(point(bX + aw + spacing, bY + bHeight))
        lineTo(point(bX + aw + spacing, bY));
        quadraticCurveTo(point(
            bX + aw + spacing + dw * (1.0 - arcSize), bY + (bHeight - sh) * arcSize), 
            point(bX + aw + spacing + dw, bY + bHeight - sh))
        lineTo(point(bX + aw + spacing + dw, bY + bHeight))
        lineTo(point(bX + aw + spacing, bY + bHeight))
        context.fillStyle = blue
        context.fill()

        // substain
        context.beginPath()
        moveTo(point(bX + aw + spacing + dw + spacing, bY + bHeight - sh))
        lineTo(point(bX + bWidth - spacing - rw, bY + bHeight - sh))
        lineTo(point(bX + bWidth - spacing - rw, bY + bHeight))
        lineTo(point(bX + aw + spacing + dw + spacing, bY + bHeight))
        context.closePath()
        context.strokeStyle = yellow
        context.stroke()

        context.beginPath()
        moveTo(point(bX + aw + spacing + dw + spacing, bY + bHeight - sh))
        lineTo(point(bX + bWidth - spacing - rw, bY + bHeight - sh))
        lineTo(point(bX + bWidth - spacing - rw, bY + bHeight))
        lineTo(point(bX + aw + spacing + dw + spacing, bY + bHeight))
        context.fillStyle = yellow
        context.fill()

        // release
        context.beginPath()
        moveTo(point(bX + bWidth - rw, bY + bHeight))
        lineTo(point(bX + bWidth - rw, bY + bHeight - sh))
        quadraticCurveTo(
            point(bX + bWidth - rw * arcSize, bY + bHeight - sh * (1.0 - arcSize)), 
            point(bX + bWidth, bY + bHeight))
        context.closePath()
        context.strokeStyle = red
        context.stroke()

        context.beginPath()
        moveTo(point(bX + bWidth - rw, bY + bHeight))
        lineTo(point(bX + bWidth - rw, bY + bHeight - sh))
        quadraticCurveTo(
            point(bX + bWidth - rw * arcSize, bY + bHeight - sh * (1.0 - arcSize)), 
            point(bX + bWidth, bY + bHeight))
        context.fillStyle = red
        context.fill()

    }

    // TODO: move to utils.js
    function clamp (v, min, max) { return v < min ? min : v > max ? max : v }
    function normalize(n, min, max) { return (n - min) / (max - min) }
    function vector(x,y) { return new Vector(x,y) }
    function point(x,y) { return new Vector(x,y) } // hmm...
    function floor(x) { return Math.floor(x) }
    function transform_scale(v, s) { return v.mul(s) }
    function translate(v,p) { return v.plus(p) }
    function rotate(p,a,orgin) {
        let s = Math.sin(a)
        let c = Math.cos(a)
        
        // translate to orgin
        p = p.minus(orgin)

        // rotate around orgin
        p = point(p.x * c - p.y * s, p.x * s + p.y * c)

        // translate back to orignal position
        return p.plus(orgin)
    }
    function radians(d) { return d * (Math.PI/180) }
    function map(x, in_min, in_max, out_min, out_max) {
        return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
    }
}