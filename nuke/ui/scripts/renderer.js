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
    this.filter = 0.1
    this.wave = 3.0
    this.relation = 1.0
    this.sub = 0.0

    this.controlWave = (v) => {
        let mv = map(v, 0, 127, 0.08, 4.0)
        this.wave = mv
        return mv
    }

    this.controlSub = (v) => {
        let mv = map(v, 0, 127, 0, 1.0)
        this.sub = mv
        return mv
    }
    
    this.controlRelation = (v) => {
        let mv = map(v, 0, 127, 0, 3.01)
        this.relation = mv
        return mv 
    }
    
    this.controlFilter = (v) => {
        let mv = map(v, 0, 127, 0, 1.0)
        this.filter = mv
        return mv
    }

    // handle Midi Twister style endless encoders
    this.incWave = (v) => {
        this.wave = clamp(this.wave + 0.01 * v, 0, 4)
    }

    this.incSub = (v) => {
        this.sub = clamp(this.sub + 0.01 * v, 0, 1)
    }

    this.incRelation = (v) => {
        this.relation = clamp(this.relation + 0.01 * v, 0, 3.01)
    }

    this.incFilter = (v) => {
        this.filter = clamp(this.filter + 0.01 * v, 0, 1)
    }

    // handle directly setting params
    this.updateFilter = function(value) {
        this.filter = value;
    }

    this.updateWave = function(value) {
        this.wave = value;
    }

    this.updateRelation = function(value) {
        this.relation = value
    }

    this.updateSub = function(value) {
        this.sub = value
    }


    // constants

    const red = "#DD4A22"
    const yellow = "#F3B83C"
    const blue = "#6060A6"
    const rose = "#FFBDB0"
    const rootbeer = "#1f0c07"

    const pw_square = new Array(
        vector(0.0, 1.0), vector(0.0, 1.0), vector(0.0, 0.0), vector(2.0, 0.0), 
        vector(2.0, 1.0), vector(2.0, 1.0), vector(2.0, 0.0), vector(4.0, 0.0), 
        vector(4.0, 1.0)
    );

    const square = [
        vector(0.0, 1.0), vector(1.0, 1.0), vector(1.0, 0.0), vector(2.0, 0.0), 
        vector(2.0, 1.0), vector(3.0, 1.0), vector(3.0, 0.0), vector(4.0, 0.0), 
        vector(4.0, 1.0)
    ];

    const below_triangle = [
        vector(4.0 / 5.0, 1.0), vector(4.0 / 5.0, 1.0),  vector(2.0 * 4.0 / 5.0, 0.0),
        vector(2.0 * 4.0 / 5.0, 0.0), vector(3.0 * 4.0 / 5.0, 1.0), vector(3.0 * 4.0 / 5.0, 1.0),
        vector(4.0 * 4.0 / 5.0, 0.0), vector(4.0 * 4.0 / 5.0, 0.0), vector(4.0, 1.0)
    ];

    const triangle = [
        vector(4.0 / 5.0, 1.0), vector(2.0 * 4.0 / 5.0, 0.0), vector(3.0 * 4.0 / 5.0, 1.0),
        vector(3.0 * 4.0 / 5.0, 1.0), vector(3.0 * 4.0 / 5.0, 1.0), vector(4.0 * 4.0 / 5.0, 0.0),
        vector(4.0, 1.0), vector(4.0, 1.0), vector(4.0, 1.0)
    ];

    const saw = [
        vector(0.0, 1.0), vector(2.0, 0.0), vector(2.0, 1.0), vector(2.0, 1.0), 
        vector(2.0, 1.0), vector(4.0, 0.0), vector(4.0, 1.0), vector(4.0, 1.0), 
        vector(4.0, 1.0)
    ];

    const hs_saw = [
        vector(0.0, 1.0), vector(1.0, 0.0), vector(1.0, 1.0), vector(2.0, 0.0), 
        vector(2.0, 1.0), vector(3.0, 0.0), vector(3.0, 1.0), vector(4.0, 0.0), 
        vector(4.0, 1.0)
    ];

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
        this.drawKey()
        this.drawFilter()
        this.drawWave()
        this.drawDial()
        client.adsr.draw(this.context, vector(300, 0))
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

    this.drawKey = function() {
        var style = { color: "#d3d3d3", thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        this.setStyle(style)

        // box
        this.context.beginPath();
        this.context.moveTo(269.3, 76.4);
        this.context.lineTo(211.0, 76.4);
        this.context.bezierCurveTo(203.0, 76.4, 196.5, 70.0, 196.5, 62.0);
        this.context.lineTo(196.5, 47.6);
        this.context.bezierCurveTo(196.5, 39.6, 203.0, 33.2, 211.0, 33.2);
        this.context.lineTo(269.3, 33.2);
        this.context.bezierCurveTo(277.3, 33.2, 283.8, 39.6, 283.8, 47.6);
        this.context.lineTo(283.8, 62.0);
        this.context.bezierCurveTo(283.8, 70.0, 277.3, 76.4, 269.3, 76.4);
        this.context.closePath();
        this.context.stroke();

        this.context.strokeStyle = yellow
        // Nuke/Key
        this.context.beginPath();
        this.context.moveTo(228.8, 54.5);
        this.context.bezierCurveTo(228.8, 59.2, 225.0, 63.0, 220.3, 63.0);
        this.context.bezierCurveTo(215.6, 63.0, 211.8, 59.2, 211.8, 54.5);
        this.context.bezierCurveTo(211.8, 49.8, 215.6, 46.0, 220.3, 46.0);
        this.context.bezierCurveTo(225.0, 46.0, 228.8, 49.8, 228.8, 54.5);
        this.context.closePath();
        this.context.stroke();

        // Nuke/Key/Base
        this.context.beginPath();
        this.context.moveTo(228.8, 54.5);
        this.context.lineTo(268.6, 54.5);
        this.context.stroke();

        // Nuke/Key/Tooth 2
        this.context.beginPath();
        this.context.moveTo(260.6, 54.5);
        this.context.lineTo(260.6, 63.0);
        this.context.stroke();

        // Nuke/Key/Tooth 1
        this.context.beginPath();
        this.context.moveTo(249.6, 54.5);
        this.context.lineTo(249.6, 63.0);
        this.context.stroke();

        // Nuke/Key/Fill
        this.context.beginPath();
        this.context.moveTo(225.4, 47.7);
        this.context.lineTo(225.4, 61.1);
        this.context.stroke();
    }

    this.drawFilter = function () {
        var style = { color: "#d3d3d3", thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        
        this.context.beginPath()
        this.context.moveTo(269.3, 199.0)
        this.context.lineTo(211.0, 199.0)
        this.context.bezierCurveTo(203.0,199.0, 196.5,192.5, 196.5,184.5)
        this.context.lineTo(196.5, 129.2 + 10.0)
        this.context.bezierCurveTo(196.5, 121.2 + 10.0, 203.0, 114.7 + 10.0, 211.0, 114.7 + 10.0)
        this.context.lineTo(269.3, 114.7 + 10.0)
        this.context.bezierCurveTo(277.3, 114.7 + 10.0, 283.8, 121.2 + 10.0, 283.8, 129.2 + 10.0)
        this.context.lineTo(283.8, 184.5)
        this.context.bezierCurveTo(283.8, 192.5, 277.3, 199.0, 269.3, 199.0)
        
        this.setStyle(style)
        this.context.stroke();
        this.context.closePath()
        
        // filter gauge
        let length = this.filter * 40.0 
        this.context.beginPath()
        this.context.moveTo(210.1, 157.2)
        this.context.lineTo(210.1 + length, 157.2 + self.filter * 5.0) 
        this.context.bezierCurveTo(221.1 + length, 164.7, 229.2 + length, 173.3, 229.7 + length, 183.8)
        this.context.bezierCurveTo(229.7 + length, 184.3, 229.8 + length, 184.7, 229.8 + length, 185.2)
        this.context.lineTo(229.8 + length, 187.3) 

        this.context.strokeStyle = red
        this.context.stroke();
        this.context.closePath()
    }

    this.drawWave = function() {
        var style = { color: "#d3d3d3", thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        this.setStyle(style)
        this.context.beginPath()
        // draw box
        this.context.moveTo(136.5, 214.5)
        this.context.lineTo(50.7, 214.5)
        this.context.bezierCurveTo(42.7, 214.5, 36.2, 208.1, 36.2, 200.1)
        this.context.lineTo(36.2, 100.7)
        this.context.bezierCurveTo(36.2, 92.7, 42.7, 86.3, 50.7, 86.3)
        this.context.lineTo(136.5, 86.3)
        this.context.bezierCurveTo(144.5, 86.3, 151.0, 92.7, 151.0, 100.7)
        this.context.lineTo(151.0, 200.1)
        this.context.bezierCurveTo(151.0, 208.1, 144.5, 214.5, 136.5, 214.5)

        // draw line between wave and hazard sign
        this.context.moveTo(55.1, 131.6)
        this.context.lineTo(132.0, 131.6)

        this.context.strokeStyle = "#d3d3d3"
        this.context.stroke()
        this.context.closePath()
        
        // draw waveform
        let dial = normalize(this.wave, 0.0, 4.0) * 4.0;
        var frac = dial - floor(dial);
        var points = pw_square;
        if (dial >= 0.0 && dial < 1.0) {
            points = pw_square.map((x, i) => {
                let y = square[i]
                return x.mul_scalar(1.0 - frac).plus(y.mul_scalar(frac))
            })
        }
        else if (dial >= 1.0 && dial < 2.0) {
            points = square.map((x,i) => {
                let y = below_triangle[i] 
                return x.mul_scalar(1.0 - frac).plus(y.mul_scalar(frac))
            })
        }
        else if (dial >= 2.0 && dial < 3.0) {
            points = triangle.map((x,i) => {
                let y = saw[i] 
                return x.mul_scalar(1.0 - frac).plus(y.mul_scalar(frac))
            })
        }
        else if (dial >= 3.0 && dial < 4.0) {
            points = saw.map((x,i) => {
                let y = hs_saw[i]
                return x.mul_scalar(1.0 - frac).plus(y.mul_scalar(frac))
            })
        }
        else if (dial == 4.0) {
            points = hs_saw.map((x, i) => x.mul_scalar(1.0))
        }

        // transform with scale and move into position
        let trans = vector(65.0 / 4.0, 12.0)
        points = points.map((v, i) => translate(transform_scale(v, trans), vector(60.0, 102.0)))
        // finally draw wave
        this.context.beginPath()
        this.context.moveTo(points[0].x, points[0].y)
        for (var i = 1; i < points.length; i++) {
            this.context.lineTo(points[i].x, points[i].y)
        }

        this.context.strokeStyle = blue
        this.context.stroke()
        this.context.closePath()

        // draw hazard
        let hazard_rotation_value_l = 0.0 + this.relation * 0.3;
        let hazard_rotation_value_r = 0.0 - this.relation * 0.3;

        // helpers
        let transform_l = (p) => rotate(p, hazard_rotation_value_l, vector(93.6, 172.6))
        let transform_r = (p) => rotate(p, hazard_rotation_value_r, vector(93.6, 172.6))
        let moveTo = (p) => this.context.moveTo(p.x, p.y)
        let lineTo = (p) => this.context.lineTo(p.x, p.y)
        let arc = (cp, r, sa, ea) => this.context.arc(cp.x, cp.y, r, sa, ea)

        this.context.beginPath()

        let centre = point(93.6, 172.6)
        let radius = 26.0
       
        var p1 = transform_l(point(93.6, 198.1))
        var p3 =transform_r(point(93.6, 198.1))
        moveTo(transform_l(point(93.6, 180.6)))
        lineTo(p1)
        moveTo(transform_r(point(93.6, 180.6)))
        lineTo(p3)
        var endAngle = Math.atan2(p1.y - centre.y, p1.x - centre.x)
        var startAngle   = Math.atan2(p3.y - centre.y, p3.x - centre.x)
        arc(centre, radius, startAngle, endAngle)
        
        p1 = transform_l(point(115.6, 159.9))
        p3 = transform_r(point(115.6, 159.9))
        moveTo(transform_l(point(100.5, 168.6)))
        lineTo(p1)
        moveTo(transform_r(point(100.5, 168.6)))
        lineTo(p3)
        endAngle = Math.atan2(p1.y - centre.y, p1.x - centre.x)
        startAngle   = Math.atan2(p3.y - centre.y, p3.x - centre.x)
        arc(centre, radius, startAngle, endAngle)
        
        p1 = transform_l(point(71.5, 159.9))
        p3 = transform_r(point(71.5, 159.9))
        moveTo(transform_l(point(86.7, 168.6)))
        lineTo(p1)
        moveTo(transform_r(point(86.7, 168.6)))
        lineTo(p3)
        endAngle = Math.atan2(p1.y - centre.y, p1.x - centre.x)
        startAngle   = Math.atan2(p3.y - centre.y, p3.x - centre.x)
        arc(centre, radius, startAngle, endAngle)
        
        // draw centre circle
        this.context.moveTo(101.6, 172.6)
        this.context.bezierCurveTo(101.6, 177.0, 98.0, 180.6, 93.6, 180.6);
        this.context.bezierCurveTo(89.2, 180.6, 85.6, 177.0, 85.6, 172.6);
        this.context.bezierCurveTo(85.6, 168.2, 89.2, 164.6, 93.6, 164.6);
        this.context.bezierCurveTo(98.0, 164.6, 101.6, 168.2, 101.6, 172.6);

        this.context.strokeStyle = rose
        this.context.stroke()
    }

    this.drawDial = function() {
        var style = { color: "#ffffff", thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        this.setStyle(style)

        this.context.beginPath()
        this.context.moveTo(59.0, 57.5)
        this.context.bezierCurveTo(59.0, 38.5, 74.5, 23.0, 93.6, 23.0)
        this.context.stroke()

        this.context.beginPath()
        this.context.moveTo(93.6, 23.0)
        this.context.bezierCurveTo(112.7, 23.0, 128.1, 38.5, 128.1, 57.5)
        this.context.strokeStyle = red
        this.context.stroke()

        this.context.beginPath()
        this.context.moveTo(95.4, 57.5)
        this.context.bezierCurveTo(95.4, 58.5, 94.6, 59.3, 93.6, 59.3)
        this.context.bezierCurveTo(92.6, 59.3, 91.8, 58.5, 91.8, 57.5)
        this.context.bezierCurveTo(91.8, 56.6, 92.6, 55.8, 93.6, 55.8)
        this.context.bezierCurveTo(94.6, 55.8, 95.4, 56.6, 95.4, 57.5)
        this.context.stroke()

        this.context.beginPath()
        let rotation = -1.57 + this.sub * Math.PI
        this.context.moveTo(93.6, 57.5)
        let p1 = rotate(point(93.6, 37.3), rotation, vector(93.6, 57.5))
        this.context.lineTo(p1.x, p1.y)
        this.context.stroke()
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