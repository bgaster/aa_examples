/*
 *
 */
'use strict'

this.hold = false
this.currentKey = 0


function Renderer (client) {
    this.el = document.createElement('canvas')
    this.el.id = 'guide'
    // set required size of canvas here
    this.el.width = 640
    this.el.height = 640
    this.el.style.width = '320px'
    this.el.style.height = '320px'
    this.context = this.el.getContext('2d')
    this.showExtras = true


    this.scale = 1 //window.devicePixelRatio

    // params
    this.param1 = 0.05

    // handle incoming control message, set values of params, etc.
    this.controlParam1 = (v) => {
        let mv = map(v, 0, 127, 0.08, 4.0)
        this.param1 = mv
        return mv
    }

    // constants
    const offwhite = "#fffddd"
    const red = "#DD4A22"
    const yellow = "#F3B83C"
    const blue = "#6060A6"
    const rose = "#FFBDB0"
    const rootbeer = "#1f0c07"
    const black = "#000000"
    const shadow = "#a2a2a2"
    const shadower = "#626262"
    const white = "#ffffff"
    const display = "#5c5c3d"
    const moreoff = "#ffffcc"
    const redd = "#ff0000"
    const orange = "#ff8600"
    const blue2 = "#0020ff"

    // functions
    this.start = function () {
        this.update()
    }

    this.update = function (force = false) {
        this.resize()
        this.draw()
    }

    this.draw = function() {
        this.clear()
        this.drawKey()
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

    this.aButton = function(x, y, colour) {
      this.context.beginPath();
      this.context.fillStyle = colour;
      this.context.fillRect(x, y, 20, 40);
      this.context.lineStyle = shadow;
      this.context.lineWidth = 2;
      this.context.moveTo(x+1, y+40);
      this.context.lineTo(x+20, y+40);
      this.context.lineTo(x+20, y+1);
      this.context.stroke();
    }

    // Animate keys
    var isWhite = true
    var animatedColour = white
    this.animateKeys = function(x, aWhite){
      this.currentKey = x
      if (aWhite == true) {
        isWhite = true
        if (this.hold == true) {
          animatedColour = shadow
        }
        if (this.hold == false) {
          animatedColour = white
        }
      }
      if (aWhite == false) {
        isWhite = false
        if (this.hold == true) {
          animatedColour = shadow
        }
        if (this.hold == false) {
          animatedColour = black
        }
      }
    }


    // Set params
    this.waveform = 0
    this.attack = 0
    this.decay = 1
    this.sustain = 2
    this.release = 1
    this.vibrato = 0
    this.tremelo = 0
    this.tempo = 0
    this.octave = 1

    this.setParams = function(param, value) {
      if (param == 0) {
        this.waveform = value
        sendWaveform(value)
      }
      if (param == 1) {
        this.attack = value
        sendAttack(value)
      }
      if (param == 2) {
        this.decay = value
        sendDecay(value)
      }
      if (param == 3) {
        this.sustain = value
        sendSustain(value)
      }
      if (param == 4) {
        this.release = value
        sendRelease(value)
      }
      if (param == 5) {
        this.vibrato = value
        sendVibrato(value)
      }
      if (param == 6) {
        this.tremelo = value
        sendTremelo(value)
      }
      if (param == 7) {
        this.tempo = value
      }
      if (param == 8) {
        this.octave = octave
      }
    }


    // ------------------

    this.drawKey = function() {
        var style = { color: offwhite, thickness: 6.0, strokeLinecap: "round", strokeLinejoin: "round"}
        this.setStyle(style)

        // outline
        this.context.strokeStyle = offwhite;
        this.context.beginPath();
        this.context.moveTo(50, 20);
        this.context.lineTo(1270, 20);
        this.context.bezierCurveTo(1270, 20, 1300, 30, 1300, 60);
        this.context.lineTo(1300, 300);
        this.context.bezierCurveTo(1300, 300, 1300, 340, 1280, 340);
        this.context.lineTo(60, 340);
        this.context.bezierCurveTo(60, 340, 40, 335, 30, 300);
        this.context.lineTo(30, 60);
        this.context.bezierCurveTo(30, 60, 30, 20, 50, 20);
        this.context.closePath();
        this.context.fillStyle = moreoff;
        this.context.fill();
        this.context.stroke();

        // speaker
        this.context.beginPath();
        this.context.strokeStyle = shadow;
        this.context.lineWidth = 3;
        this.context.moveTo(55, 45);
        for (var x = 0; x < 28; x++) {
          this.context.lineTo(255, (x*10)+45);
          this.context. moveTo(55, ((x+1)*10)+45);
        }
        this.context.stroke();
        this.context.beginPath();
        this.context.strokeStyle = black;
        this.context.lineWidth = 3;
        this.context.strokeRect(55, 35, 200, 290);

        this.context.beginPath();
        this.context.strokeStyle = black;
        this.context.lineWidth =2;
        this.context.moveTo(55, 43);
        for (var x = 0; x < 28; x++) {
          this.context.lineTo(255, (x*10)+43);
          this.context. moveTo(55, ((x+1)*10)+43);
        }
        this.context.stroke();

        //Screen
        this.context.fillStyle = black;
        this.context.fillRect(285, 16, 270, 140);
        this.context.stroke();
        this.context.fillStyle = display;
        this.context.fillRect(310, 60, 220, 60);
        this.context.stroke();

        this.context.beginPath();
        this.context.strokeStyle = black;
        this.context.lineWidth = 1;
        this.context.moveTo(310, 62);
        this.context.lineTo(530,62);
        this.context.stroke();
        this.context.beginPath();
        this.context.strokeStyle = white;
        this.context.lineWidth = 2;
        this.context.moveTo(285, 20);
        this.context.lineTo(555,20);
        this.context.moveTo(285, 24);
        this.context.lineTo(555,24);
        this.context.stroke();
        this.context.beginPath();
        this.context.strokeStyle = shadow;
        this.context.lineWidth = 4;
        this.context.moveTo(286, 30);
        this.context.lineTo(554,30);
        this.context.stroke();
        this.context.beginPath()
        this.context.lineWidth = 2;
        this.context.moveTo(286, 150);
        this.context.lineTo(554, 150);
        this.context.stroke();

        this.context.font = "bold 14px Arial"
        this.context.fillStyle = shadow;
        this.context.fillText("CASIO", 370, 50);
        this.context.stroke();
        this.context.font = "11px Arial"
        this.context.fillStyle = shadow;
        this.context.fillText("VL-TONE", 420, 50);
        this.context.stroke();
        this.context.font = "8px Arial"
        this.context.fillStyle = shadow;
        this.context.fillText("ELECTRONIC   MUSICAL   INSTRUMENT     VL-1", 335, 140);
        this.context.stroke();


        // key bed
        this.context.beginPath();
        this.context.strokeStyle = shadow;
        this.context.lineWidth =3;
        this.context.moveTo(285, 173);
        this.context.lineTo(1304, 173);
        this.context.moveTo(288, 173);
        this.context.lineTo(288, 343);
        this.context.stroke();
        this.context.lineWidth = 2;
        this.context.moveTo(307, 173);
        this.context.lineTo(307, 343);
        this.context.moveTo(1285, 173);
        this.context.lineTo(1285, 341);
        this.context.stroke();
        this.context.beginPath();
        this.context.strokeStyle = black;
        this.context.lineWidth =3;
        this.context.moveTo(285, 345);
        this.context.lineTo(285, 170);
        this.context.lineTo(1305, 170);
        this.context.stroke();

        // keys
        this.context.beginPath();
        this.context.fillStyle = white;
        var notes = ['G1', 'A1', 'B1', 'C2', 'D2', 'E2', 'F2', 'G2', 'A2', 'B2', 'C3', 'D3', 'E3', 'F3', 'G3', 'A3', 'B3'];
        for (var x = 0; x < notes.length; x++) {
          this.context.fillStyle = white;
          this.context.fillRect(325 + (x*57), 255, 30, 60);
          if (notes[x].includes("G") || notes[x].includes("A") || notes[x].includes("C") || notes[x].includes("D") || notes[x].includes("F")) {
            this.context.fillStyle = black;
            this.context.fillRect(356 + (x*57), 170, 25, 70);
          }
          if (x == this.currentKey && isWhite == true){
            this.context.fillStyle = animatedColour;
            this.context.fillRect(325 + (x*57), 255, 30, 60);
          }
          if (x == this.currentKey && isWhite == false){
            this.context.fillStyle = animatedColour;
            this.context.fillRect(356 + (x*57), 170, 25, 70);
          }
        }
        for (var x = 0; x < notes.length; x++) {
          this.context.strokeStyle = shadow;
          this.context.lineWidth = 2;
          var aPoint = 325 + (x*57);
          this.context.moveTo(aPoint, 315);
          this.context.lineTo(aPoint + 30, 315);
          this.context.lineTo(aPoint + 30, 255);
          if (notes[x].includes("G") || notes[x].includes("A") || notes[x].includes("C") || notes[x].includes("D") || notes[x].includes("F")) {
            this.context.moveTo((357 + (x*57)), 241);
            this.context.lineTo((357 + (x*57)) + 25, 241);
            this.context.lineTo((357 + (x*57)) + 25, 172);
          }
          this.context.stroke();
        }

        // Interface
        var buttonXs = [[570,redd],[600,redd],[660,white],[690,white],[730,white],[760,white],[800,white],[830,white],[870,white],[900,white],[960,orange],[990,orange],[1040,blue2],[1070,blue2],[1120,rose],[1150,rose]];
        for (var x  = 0; x < buttonXs.length; x++) {
          this.context.strokeStyle = shadow
          this.context.lineWidth = 2
          this.aButton(buttonXs[x][0], 100, buttonXs[x][1]);
          this.context.beginPath();
          this.context.strokeStyle = black
          this.context.lineWidth = 1
          this.context.moveTo(buttonXs[x][0] + 5, 120);
          this.context.lineTo(buttonXs[x][0] + 15, 120);
          if (x % 2 != 0) {
            this.context.moveTo(buttonXs[x][0] + 10, 115);
            this.context.lineTo(buttonXs[x][0] + 10, 125);
          }
          this.context.stroke();
        }

        this.context.beginPath();
        this.context.fillStyle = shadow;
        this.context.strokeStyle = shadower
        this.context.lineWidth = 2;
        for (var x  = 0; x <5; x++) {
          this.context.fillRect(585 + 70*x, 45, 40, 20);
          this.context.moveTo(586+70*x, 45+20);
          this.context.lineTo(625+70*x, 45+20);
          this.context.lineTo(625+70*x, 45+1);
          this.context.stroke();
        }

        this.context.beginPath()
        this.context.fillRect(1020, 45, 40, 20);
        this.context.moveTo(1020, 45+20);
        this.context.lineTo(1060, 45+20);
        this.context.lineTo(1060, 45+1);
        this.context.fillRect(1080, 45, 40, 20);
        this.context.moveTo(1080, 45+20);
        this.context.lineTo(1120, 45+20);
        this.context.lineTo(1120, 45+1);
        this.context.stroke();
        this.context.beginPath();
        this.context.moveTo(1020 + 15, 55);
        this.context.lineTo(1020 + 25, 55);
        this.context.moveTo(1085 + 15, 50);
        this.context.lineTo(1085 + 15, 60);
        this.context.moveTo(1080 + 15, 55);
        this.context.lineTo(1080 + 25, 55);


        // Interface text
        this.context.font = "11px Arial"
        this.context.fillStyle = black;
        this.context.fillText("Waveform", buttonXs[0][0], 157);
        this.context.fillText("Attack", buttonXs[2][0]+12, 157);
        this.context.fillText("Decay", buttonXs[4][0]+11, 157);
        this.context.fillText("Sustain", buttonXs[6][0]+9, 157);
        this.context.fillText("Release", buttonXs[8][0]+8, 157);
        this.context.fillText("Vibrato", buttonXs[10][0]+8, 157);
        this.context.fillText("Tremelo", buttonXs[12][0]+7, 157);
        this.context.fillText("Tempo", buttonXs[14][0]+11, 157);
        this.context.fillText("Volume", 1217, 157)
        this.context.fillText("Piano", 593, 80);
        this.context.fillText("Fantasy", 656, 80);
        this.context.fillText("Violin", 732, 80);
        this.context.fillText("Flute", 803, 80);
        this.context.fillText("Guitar", 871, 80);
        this.context.fillText("Octave", 1052, 80);
        this.context.stroke();



        // Screen Text
        this.context.font = "40px Arial"
        this.context.fillStyle = black;
        this.context.fillText(this.waveform, 320, 108)
        this.context.fillText(this.attack, 350, 108)
        this.context.fillText(this.decay, 380, 108)
        this.context.fillText(this.sustain, 410, 108)
        this.context.fillText(this.release, 440, 108)
        this.context.fillText(this.vibrato, 470, 108)
        this.context.fillText(this.tremelo, 500, 108)
        this.context.stroke();

        // Volume slider
        this.context.beginPath();
        this.context.lineWidth = 7
        this.context.strokeStyle = black
        this.context.moveTo(1235, 138);
        this.context.lineTo(1235, 44);
        this.context.stroke();

        this.context.beginPath()
        this.context.fillStyle = shadow
        this.context.fillRect(1225, 54, 20, 30)
        this.context.lineWidth = 1
        this.context.strokeStyle = black
        this.context.moveTo(1225, 54 + 15)
        this.context.lineTo(1245, 54 + 15)
        this.context.moveTo(1225, 54 + 10)
        this.context.lineTo(1245, 54 + 10)
        this.context.moveTo(1225, 54 + 20)
        this.context.lineTo(1245, 54 + 20)
        this.context.stroke()


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

    // some helper functions

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
