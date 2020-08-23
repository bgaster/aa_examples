/*
 * 
 */
'use strict'

function Renderer (client) {
    this.el = document.createElement('canvas')
    this.el.id = 'guide'
    this.el.width = 640
    this.el.height = 640
    this.el.style.width = '320px'
    this.el.style.height = '320px'
    this.context = this.el.getContext('2d')
    this.showExtras = true

    this.scale = 1 //window.devicePixelRatio

    // params

    // constants

    const red = "#DD4A22"
    const yellow = "#F3B83C"
    const blue = "#6060A6"
    const rose = "#FFBDB0"
    const rootbeer = "#1f0c07"

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
        this.drawDots()
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

    this.drawDots = function() {
        var style = { color: "#d3d3d3", thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        this.setStyle(style)
        
        this.context.beginPath()
        this.context.moveTo(286.6, 29.2)
        this.context.lineTo(287.0, 29.2)
        this.context.stroke()

        this.context.moveTo(193.4, 29.2);
        this.context.lineTo(193.7, 29.2);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(286.6, 79.4);
        this.context.lineTo(287.0, 79.4);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(193.4, 79.4);
        this.context.lineTo(193.7, 79.4);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(286.6, 110.7);
        this.context.lineTo(287.0, 110.7);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(193.4, 110.7);
        this.context.lineTo(193.7, 110.7);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(286.6, 202.0);
        this.context.lineTo(287.0, 202.0);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(193.4, 202.0);
        this.context.lineTo(193.7, 202.0);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(153.8, 217.5);
        this.context.lineTo(154.1, 217.5);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(33.0, 217.5);
        this.context.lineTo(33.4, 217.5);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(153.8, 82.3);
        this.context.lineTo(154.1, 82.3);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(33.0, 82.3);
        this.context.lineTo(33.4, 82.3);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(153.8, 22.3);
        this.context.lineTo(154.1, 22.3);
        this.context.stroke();

        this.context.beginPath();
        this.context.moveTo(33.0, 22.3);
        this.context.lineTo(33.4, 22.3);
        this.context.stroke();
    }

// ------------------


    this.drawVertex = function (pos, radius = 5) {
        this.context.beginPath()
        this.context.lineWidth = 2
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
}