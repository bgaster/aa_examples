/*
 * 
 */
'use strict'

function Renderer (client) {
    this.el = document.createElement('canvas')
    this.el.id = 'guide'
    this.el.width = 380
    this.el.height = 320
    this.el.style.width = '380px'
    this.el.style.height = '320px'
    this.context = this.el.getContext('2d')
    this.showExtras = true

    this.scale = 1 //window.devicePixelRatio

    // params
    this.shape_ = 1.0
    this.spread_ = 0.0
    this.length_ = 0.5
    this.shimmer_ = 0.1
    this.dryWet_ = 0.5

    this.controlShape = (v) => {
        let mv = map(v, 0, 127, 0.0, 2.0)
        this.shape_ = mv
        return mv
    }

    this.controlSpread = (v) => {
        let mv = map(v, 0, 127, 0, 1.0)
        this.spread_ = mv
        return mv
    }
    
    this.controlLength = (v) => {
        let mv = map(v, 0, 127, 0, 1.2)
        this.length_ = mv
        return mv 
    }
    
    this.controlShimmer = (v) => {
        let mv = map(v, 0, 127, 0, 1.2)
        this.shimmer_ = mv
        return mv
    }

    this.controlDryWet = (v) => {
        let mv = map(v, 0, 127, 0, 1.0)
        this.dryWet_ = mv
        return mv
    }

    // handle Midi Twister style endless encoders
    this.incShape = (v) => {
        this.shape_ = clamp(this.shape_ + 0.01 * v, 0, 2.0)
    }

    this.incSpread = (v) => {
        this.spread_ = clamp(this.spread_ + 0.01 * v, 0, 1.0)
    }

    this.incLength = (v) => {
        this.length_ = clamp(this.relation + 0.01 * v, 0, 1.2)
    }

    this.incShimmer = (v) => {
        this.shimmer_ = clamp(this.filter + 0.01 * v, 0, 1.2)
    }

    this.updateShape = (v) => {
        this.shape_ = v
    }

    this.updateSpread = (v) => {
        this.spread_ = v
    }

    this.updateLength = (v) => {
        this.length_ = v
    }

    this.updateShimmer = (v) => {
        this.shimmer_ = v
    }

    // constants

    const red = "#DD4A22"
    const yellow = "#F3B83C"
    const blue = "#6060A6"
    const rose = "#FFBDB0"
    const rootbeer = "#1f0c07"
    const white = "#FFFFFF"

    // functions 

    this.start = function () {
        this.update()
        //window.requestAnimationFrame(() => { this.update() });
    }

    this.update = function (force = false) {
        this.resize()
        this.draw()
        //window.requestAnimationFrame(() => { this.update() });
    }

    this.draw = function() {
        this.clear()
        this.drawReverb()
    }

    this.clear = function () {
        this.context.clearRect(0, 0, this.el.width * this.scale, this.el.height * this.scale);
        this.context.rect(0, 0, this.el.width * this.scale, this.el.height * this.scale);
        this.context.fillStyle = rootbeer;
        this.context.fill();
    }

    this.resize = function () {
        const _target = client.getPaddedSize()
        const _current = { width: this.el.width / this.scale, height: this.el.height / this.scale }
        const offset = sizeOffset(_target, _current)
        if (offset.width === 0 && offset.height === 0) {
            return
        }
        console.log('Renderer', `Require resize: ${printSize(_target)}, from ${printSize(_current)}`)
        this.el.width = (_target.width) * this.scale
        this.el.height = (_target.height) * this.scale
        this.el.style.width = (_target.width) + 'px'
        this.el.style.height = (_target.height) + 'px'
    }
    // ------------------

    
    this.drawReverb = function() {
        var style = { color: "#d3d3d3", thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        this.setStyle(style)

        // draw the stars
        let star_radius = 3.0
        let shimmer =  normalize(this.shimmer_, 0.0, 1.2)
        //dim(yellow, 1.0 - shimmer)
        this.context.fillStyle = dim(yellow, 1.0 - shimmer)
        this.drawVertex(point(126.6, 75.2), star_radius)
        this.drawVertex(point(228.8, 162.3), star_radius)
        
        this.drawVertex(point(186.8, 81.3), star_radius)
        this.drawVertex(point(259.0, 28.3), star_radius)
        this.drawVertex(point(207.6, 192.4), star_radius)
        this.drawVertex(point(154.8, 93.3), star_radius)
        this.drawVertex(point(144.3, 26.3), star_radius)
        this.drawVertex(point(257.8, 193.0), star_radius)
        this.drawVertex(point(223.5, 128.3), star_radius)
        this.drawVertex(point(294.1, 181.6), star_radius)
        this.drawVertex(point(52.7, 190.3), star_radius)
        this.drawVertex(point(106.6, 42.2), star_radius)

        // next set
        second_star_radius = 2.0;
        this.drawVertex(point(255.3, 168.8), star_radius)
        this.drawVertex(point(149.5, 60.6), star_radius)
        this.drawVertex(point(200.6, 164.6), star_radius)
        this.drawVertex(point(77.6, 164.7), star_radius)
        this.drawVertex(point(175.2, 48.8), star_radius)
        this.drawVertex(point(228.0, 147.9), star_radius)
        this.drawVertex(point(228.6, 206.1), star_radius)
        this.drawVertex(point(294.0, 106.1), star_radius)
        this.drawVertex(point(159.3, 112.0), star_radius)
        this.drawVertex(point(289.7, 202.0), star_radius)
        this.drawVertex(point(45.9, 164.4), star_radius)
        this.drawVertex(point(100.4, 212.8), star_radius)
        this.drawVertex(point(100.5, 65.7), star_radius)
        this.drawVertex(point(213.7, 83.2), star_radius)
        this.drawVertex(point(213.0, 91.9), star_radius)
        this.drawVertex(point(273.0, 68.0), star_radius)
        this.drawVertex(point(255.0, 118.0), star_radius)
        this.drawVertex(point(220.0, 57.0), star_radius)
        this.drawVertex(point(200.0, 87.0), star_radius)
        this.drawVertex(point(203.0, 142.0), star_radius)
        this.drawVertex(point(113.0, 123.0), star_radius)
        this.drawVertex(point(106.0, 176.0), star_radius)
        this.drawVertex(point(159.0, 164.0), star_radius)
        this.drawVertex(point(154.0, 190.0), star_radius)
        this.drawVertex(point(182.0, 185.0), star_radius)

        // blue dot
        this.context.fillStyle = blue
        this.drawVertex(point(241.1, 85.3), 6.0)

        // blue line
        this.context.strokeStyle = blue
        this.context.beginPath()
        this.context.moveTo(144.3, 151.2)
        this.context.lineTo(241.1, 85.3)
        this.context.stroke()

        let length =  normalize(this.length_, 0.0, 1.0)
        let shape = normalize(this.shape_, 0.0, 2.0)

        let direction  = vector(3.0, -2.0).mul_scalar(length * 3.0 + 0.6)

        let minscale = 0.55;
        let backscale = Math.min(1.0, minscale + shape * (1.0 - minscale) * 2.0);
        let frontscale = Math.min(1.0, minscale + (1.0 - shape) * (1.0 - minscale) * 2.0);

        this.context.lineWidth = (style.thickness * this.scale) / backscale 

        // draw back pad
        this.context.save()
        this.context.strokeStyle = white
        let d = direction.mul_scalar(5)
        this.context.translate(d.x, d.y)
        this.context.translate(192.714, 118.458)
        this.context.scale(backscale, backscale)
        this.context.translate(-192.714, -118.458)

        this.context.beginPath()
        this.context.moveTo(159.0, 103.6)
        this.context.bezierCurveTo(159.0, 79.0, 172.3, 68.2, 192.7, 78.2)
        this.context.bezierCurveTo(213.1, 88.2, 226.4, 108.7, 226.4, 135.3)
        this.context.bezierCurveTo(226.4, 161.9, 213.9, 169.8, 192.7, 159.0)

        this.context.moveTo(226.4, 174.1)
        this.context.lineTo(226.4, 95.0)
        this.context.lineTo(159.0, 60.8)
        this.context.lineTo(159.0, 103.6)
        this.context.stroke()

        this.context.fillStyle = white
        this.context.beginPath()
        this.context.moveTo(226.4, 176.1)
        this.context.lineTo(226.4, 95.0)
        this.context.lineTo(159.0, 60.8)
        this.context.lineTo(159.0, 103.6)
        this.context.bezierCurveTo(159.0, 79.0, 172.3, 68.2, 192.7, 78.2)
        this.context.bezierCurveTo(213.1, 88.2, 226.4, 108.7, 226.4, 135.3)
        this.context.bezierCurveTo(226.4, 161.9, 213.9, 169.8, 192.7, 159.0)
        this.context.lineTo(226.4, 176.1)
        this.context.closePath()
        this.context.fill()
        this.context.restore()

        // draw back outter ring
        
        this.context.save()
        this.context.beginPath()
        this.context.strokeStyle = rose
        d = direction.mul_scalar(4.7)
        this.context.translate(d.x, d.y)
        this.context.translate(192.714, 118.458)
        this.context.scale(backscale, backscale)
        this.context.translate(-192.714, -118.458)

        this.context.moveTo(170.3, 113.6)
        this.context.bezierCurveTo(170.1, 111.6, 169.9, 109.6, 169.9, 107.6)
        this.context.bezierCurveTo(169.9, 91.1, 178.9, 83.7, 192.7, 90.4)
        this.context.bezierCurveTo(206.5, 97.2, 215.5, 111.1, 215.5, 129.0)
        this.context.bezierCurveTo(215.5, 147.20, 207.0, 152.3, 192.7, 145.1)
        this.context.bezierCurveTo(191.2, 144.3, 189.7, 143.4, 188.3, 142.5)
        this.context.stroke()
        this.context.restore()

        // single path for the rings and then simple transpose the resulting verts
        let ring = (s) => {
            this.context.save()
            this.context.strokeStyle = rose
            let d = direction.mul_scalar(s)
            this.context.translate(d.x, d.y)
            this.context.beginPath()
            this.context.moveTo(182.3, 113.6)
            this.context.bezierCurveTo(182.3, 106.1, 186.4, 102.7, 192.7, 105.8)
            this.context.bezierCurveTo(199.0, 108.9, 203.1, 115.2, 203.1, 123.5)
            this.context.bezierCurveTo(203.1, 131.7, 199.3, 134.1, 192.7, 130.8)
            this.context.stroke()
            this.context.restore()
        }

        // draw middle rings
        ring(-1.0)
        ring(1.0)
        ring(3.2)

        this.context.lineWidth = (style.thickness * this.scale) / frontscale 
        // draw front ring
        this.context.save()
        this.context.beginPath()
        this.context.strokeStyle = rose
        d = direction.mul_scalar(-4.4)
        this.context.translate(d.x, d.y)
        this.context.translate(192.714, 118.458)
        this.context.scale(frontscale, frontscale)
        this.context.translate(-192.714, -118.458)

        this.context.moveTo(205.4, 124.2)
        this.context.bezierCurveTo(205.4, 134.2, 200.7, 137.2, 192.7, 133.1)
        this.context.bezierCurveTo(184.7, 129.1, 180.0, 121.4, 180.0, 112.2)
        this.context.bezierCurveTo(180.0, 103.0, 185.0, 98.9, 192.7, 102.7)
        this.context.bezierCurveTo(200.4, 106.5, 205.4, 114.2, 205.4, 124.2)
        this.context.stroke()
        this.context.restore()

        this.context.save()
        this.context.beginPath()
        this.context.strokeStyle = rose
        d = direction.mul_scalar(-4.8)
        this.context.translate(d.x, d.y)
        this.context.translate(192.714, 118.458)
        this.context.scale(frontscale, frontscale)
        this.context.translate(-192.714, -118.458)
        this.context.moveTo(215.5, 129.0)
        this.context.bezierCurveTo(215.5, 147.0, 207.0, 152.3, 192.7, 145.1)
        this.context.bezierCurveTo(178.4, 137.8, 169.9, 124.1, 169.9, 107.6)
        this.context.bezierCurveTo(169.9, 91.1, 178.9, 83.7, 192.7, 90.4)
        this.context.bezierCurveTo(206.5, 97.2, 215.5, 111.1, 215.5, 129.0)
        this.context.stroke()
        this.context.restore()


        // draw front ring
        this.context.save()
        this.context.beginPath()
        this.context.strokeStyle = white
        this.context.fillStyle = white
        d = direction.mul_scalar(-5.0)
        this.context.translate(d.x, d.y)
        this.context.translate(192.714, 118.458)
        this.context.scale(frontscale, frontscale)
        this.context.translate(-192.714, -118.458)

        this.context.moveTo(192.7, 158.6)
        this.context.lineTo(226.4, 175.7)
        this.context.lineTo(226.4, 134.9)
        this.context.stroke()

        this.context.beginPath()
        this.context.moveTo(192.7, 158.6)
        this.context.lineTo(226.4, 175.7)
        this.context.lineTo(226.4, 134.9)
        this.context.bezierCurveTo(226.4, 161.5, 213.9, 169.4, 192.7, 158.6)
        this.context.closePath()
        this.context.fill()

        this.context.beginPath()
        this.context.moveTo(159.0, 60.4)
        this.context.lineTo(159.0, 103.2)
        this.context.moveTo(226.4, 134.9)
        this.context.lineTo(226.4, 94.6)
        this.context.lineTo(159.0, 60.4)
        this.context.stroke()

        this.context.beginPath()
        this.context.moveTo(159.0, 60.4)
        this.context.lineTo(159.0, 103.2)
        this.context.bezierCurveTo(159.0, 78.8, 172.3, 67.8, 192.7, 77.8)
        this.context.bezierCurveTo(213.1, 87.9, 226.4, 108.3, 226.4, 134.9)
        this.context.lineTo(226.4, 94.6)
        this.context.lineTo(159.0, 60.4)
        this.context.fill()

        this.context.beginPath()
        this.context.moveTo(159.0, 103.2)
        this.context.lineTo(159.0, 141.6)
        this.context.lineTo(192.7, 158.6)
        this.context.stroke()

        this.context.beginPath()
        this.context.moveTo(192.7, 158.6)
        this.context.bezierCurveTo(171.5, 147.9, 159.0, 127.6, 159.0, 103.2)
        this.context.lineTo(159.0, 141.6)
        this.context.lineTo(192.7, 158.6)
        this.context.fill()

        this.context.restore()

        // fix up line, otherwise it is drawn behind
        this.context.save()
        this.context.strokeStyle = blue
        let translation = vector(177.828 + 12.0, 128.387 - 8.0).plus(direction.mul_scalar(-4.4))
        
        if (translation.x > 144.3) {
            this.context.beginPath()
            this.context.moveTo(144.3, 151.2)
            this.context.lineTo(translation.x, translation.y)
            this.context.stroke()
        }

        this.drawVertex(point(144.3, 151.2), 6.0) 
        this.context.restore()

        this.context.font = '16px serif'
        this.context.fillStyle = white
        this.context.fillText('spread', 14.0, 45.0)
        this.context.font = '26px serif'
        this.context.fillStyle = blue
        this.context.fillText(Math.round(this.spread_*100.0), 14.0, 80.0)

        this.context.font = '16px serif'
        this.context.fillStyle = white
        this.context.fillText('dry/wet', 14.0, 115.0)
        this.context.font = '26px serif'
        this.context.fillStyle = blue
        this.context.fillText(Math.round(this.dryWet_*100.0), 14.0, 150)
    }

    
// ------------------


    this.drawVertex = function (pos, radius = 5) {
        this.context.beginPath()
        this.context.arc((pos.x * this.scale), (pos.y * this.scale), radius, 0, 2 * Math.PI, false)
        this.context.fill()
        this.context.closePath()
    }

    this.setStyle = function(style) {
        this.context.strokeStyle = style.color
        this.context.lineWidth = style.thickness * this.scale
        this.context.lineCap = style.strokeLinecap
        this.context.lineJoin = style.strokeLinejoin
    }

    this.drawPath = function (path, style) {
        const p = new Path2D(path)

        this.context.strokeStyle = style.color
        this.context.lineWidth = style.thickness * this.scale
        this.context.lineCap = style.strokeLinecap
        this.context.lineJoin = style.strokeLinejoin

        if (style.fill && style.fill !== 'none') {
            this.context.fillStyle = style.color
            this.context.fill(p)
        }

        // Dash
        this.context.save()
        if (style.strokeLineDash) { 
            this.context.setLineDash(style.strokeLineDash) 
        } 
        else { 
            this.context.setLineDash([]) 
        }
        this.context.stroke(p)
        this.context.restore()
    }

    function printSize (size) { return `${size.width}x${size.height}` }
    function sizeOffset (a, b) { return { width: a.width - b.width, height: a.height - b.height } }
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
    function hexToRgb(hex) {
        var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
          r: parseInt(result[1], 16),
          g: parseInt(result[2], 16),
          b: parseInt(result[3], 16)
        } : null;
    }
    function rgbToHex(r, g, b) {
        return "#" + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
    }
    function dim(color, amount)  {
        let dim = clamp(1.0 - amount, 0.0, 1.0)
        let c = hexToRgb(color)
        return "rgba(" + c.r + "," + c.g + "," + c.b + "," + dim + ")"
        //return rgbToHex(c.r * dim, c.g * dim, c.b * dim)
    }
}